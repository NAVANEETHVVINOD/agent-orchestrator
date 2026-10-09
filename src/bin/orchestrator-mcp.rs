use agent_orchestrator::mcp::{BoundedTransport, PlanningServer};
use rmcp::ServiceExt;
use std::sync::atomic::Ordering;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if std::env::args_os().len() != 1 {
        eprintln!("orchestrator-mcp accepts no arguments; use local stdio transport");
        std::process::exit(2);
    }
    let transport = BoundedTransport::new(tokio::io::stdin(), tokio::io::stdout());
    let failed = transport.failure_flag();
    match tokio::time::timeout(Duration::from_secs(15), PlanningServer.serve(transport)).await {
        Ok(Ok(service)) => {
            let result = service.waiting().await;
            let service_failed =
                result.is_err() || matches!(result, Ok(rmcp::service::QuitReason::JoinError(_)));
            if failed.load(Ordering::Acquire) || service_failed {
                eprintln!("Local MCP session failed");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Local MCP initialization failed or timed out");
            std::process::exit(1);
        }
    }
}
