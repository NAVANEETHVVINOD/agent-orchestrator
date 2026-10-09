//! Actual compiled-binary MCP journeys. Plans/check claims are synthetic fixtures;
//! these tests verify the local planning protocol, not Fusion or agent execution.
use agent_orchestrator::mcp::{BoundedTransport, FRAME_LIMIT, FRAME_TIMEOUT, RECORD_LIMIT};
use rmcp::{
    RoleClient, ServiceExt,
    model::{
        CallToolRequestParams, CallToolResult, ClientConfig, ContentBlock, JsonRpcMessage,
        ProtocolVersion, RequestId, ServerResult,
    },
    service::RunningService,
    transport::{TokioChildProcess, Transport},
};
use serde_json::{Value, json};
use std::{
    io,
    pin::Pin,
    process::Stdio,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    time::timeout,
};

const LIMIT: Duration = Duration::from_secs(20);
type Client = RunningService<RoleClient, ClientConfig>;
async fn client() -> Client {
    let transport =
        TokioChildProcess::new(Command::new(env!("CARGO_BIN_EXE_orchestrator-mcp"))).unwrap();
    timeout(
        LIMIT,
        ClientConfig::default()
            .with_protocol_version(ProtocolVersion::V_2025_06_18)
            .serve(transport),
    )
    .await
    .unwrap()
    .unwrap()
}
async fn call(client: &Client, name: &str, arguments: Value) -> rmcp::model::CallToolResult {
    timeout(
        LIMIT,
        client.call_tool(
            CallToolRequestParams::new(name.to_owned())
                .with_arguments(arguments.as_object().unwrap().clone()),
        ),
    )
    .await
    .unwrap()
    .unwrap()
}
async fn good(client: &Client, name: &str, arguments: Value) -> Value {
    let result = call(client, name, arguments).await;
    assert_eq!(result.is_error, Some(false), "{result:?}");
    let value = result.structured_content.unwrap();
    for flag in [
        "execution_performed",
        "external_action_authorized",
        "reported_evidence_verified",
    ] {
        assert_eq!(value[flag], false);
    }
    value
}
fn plan() -> Value {
    json!({"format_version":1,"project_id":"synthetic-mcp-journey","revision":0,"filesystem_case":"sensitive",
        "decisions":[{"id":"requirements","status":"resolved"}],
        "capabilities":[{"id":"native-builder","kind":"native-agent"}],
        "tasks":[{"id":"build","title":"Synthetic | <script> [link](https://invalid.example)","owner":"builder",
        "kind":"implementation","status":"planned","depends_on":[],"capabilities":["native-builder"],
        "requires_decisions":["requirements"],"reads":[],"writes":["src/example.rs"],"source_revision":"source-1",
        "required_checks":["qa"],"checks":[],"findings":[],"artifacts":[],"review":null}]})
}
fn event(plan: &Value, from: &str, to: &str, actor: &str) -> Value {
    json!({"format_version":1,"project_id":plan["project_id"],"expected_revision":plan["revision"],
        "task_id":"build","actor":actor,"from":from,"to":to})
}
async fn transition(client: &Client, plan: &Value, event: &Value) -> Value {
    good(
        client,
        "apply_transition",
        json!({"plan_json":plan.to_string(),"event_json":event.to_string()}),
    )
    .await["plan"]
        .clone()
}

#[tokio::test(flavor = "current_thread")]
async fn sdk_client_initializes_lists_all_tools_and_completes_review_fix_acceptance() {
    let client = client().await;
    assert_eq!(
        client
            .peer_info()
            .unwrap()
            .server_info
            .as_ref()
            .unwrap()
            .name,
        "agent-orchestrator-local-planning"
    );
    let listing = timeout(LIMIT, client.list_tools(None))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        listing
            .tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "validate_plan",
            "next_wave",
            "render_chart",
            "apply_transition",
            "route_proposal",
            "validate_workflow_config"
        ]
    );
    for tool in listing.tools {
        let annotations = tool.annotations.unwrap();
        assert_eq!(annotations.read_only_hint, Some(true));
        assert_eq!(annotations.destructive_hint, Some(false));
        assert_eq!(annotations.open_world_hint, Some(false));
        assert_eq!(tool.input_schema["additionalProperties"], false);
    }
    let mut current = plan();
    let validated = good(
        &client,
        "validate_plan",
        json!({"plan_json":current.to_string()}),
    )
    .await;
    assert_eq!(validated["record_validation"], "passed");
    let runtime = json!({"format_version":1,"project_id":current["project_id"],"revision":0,
        "free_worker_slots":1,"available_capabilities":["native-builder"],"active_tasks":[]});
    assert_eq!(
        good(
            &client,
            "next_wave",
            json!({"plan_json":current.to_string(),"runtime_json":runtime.to_string()})
        )
        .await["selected_tasks"],
        json!(["build"])
    );
    let chart = good(
        &client,
        "render_chart",
        json!({"plan_json":current.to_string()}),
    )
    .await;
    assert!(!chart["markdown"].as_str().unwrap().contains("<script>"));
    assert!(!chart["markdown"].as_str().unwrap().contains("[link]("));
    current = transition(
        &client,
        &current,
        &event(&current, "planned", "running", "coordinator"),
    )
    .await;
    let mut submitted = event(&current, "running", "review", "builder");
    submitted["checks"] = json!([{"id":"qa","status":"passed","source_revision":"source-1"}]);
    submitted["artifacts"] = json!(["docs/quality/synthetic.txt"]);
    current = transition(&client, &current, &submitted).await;
    let mut self_review = event(&current, "review", "accepted", "builder");
    self_review["review"] =
        json!({"actor":"builder","status":"passed","source_revision":"source-1"});
    assert_eq!(
        call(
            &client,
            "apply_transition",
            json!({"plan_json":current.to_string(),"event_json":self_review.to_string()})
        )
        .await
        .is_error,
        Some(true)
    );
    let mut stale = event(&current, "review", "accepted", "reviewer");
    stale["expected_revision"] = json!(0);
    stale["review"] = json!({"actor":"reviewer","status":"passed","source_revision":"source-1"});
    assert_eq!(
        call(
            &client,
            "apply_transition",
            json!({"plan_json":current.to_string(),"event_json":stale.to_string()})
        )
        .await
        .is_error,
        Some(true)
    );
    let mut fix = event(&current, "review", "needs-fix", "reviewer");
    fix["findings"] = json!(["Synthetic regression finding"]);
    current = transition(&client, &current, &fix).await;
    assert_eq!(current["tasks"][0]["checks"], json!([]));
    assert!(current["tasks"][0]["review"].is_null());
    current = transition(
        &client,
        &current,
        &event(&current, "needs-fix", "running", "builder"),
    )
    .await;
    let mut corrected = event(&current, "running", "review", "builder");
    corrected["checks"] = json!([{"id":"qa","status":"passed","source_revision":"source-1"}]);
    corrected["findings"] = json!([]);
    current = transition(&client, &current, &corrected).await;
    let mut accept = event(&current, "review", "accepted", "reviewer");
    accept["review"] = json!({"actor":"reviewer","status":"passed","source_revision":"source-1"});
    current = transition(&client, &current, &accept).await;
    assert_eq!(current["tasks"][0]["status"], "accepted");
    assert_eq!(current["revision"], 6);
    let proposal = r#"{"format_version":1,"available_routes":["qa","research"],"minimum_score":0.7,"minimum_margin":0.2,"proposals":[{"route":"qa","score":0.699999999999999999},{"route":"research","score":0.1}]}"#;
    let route = good(&client, "route_proposal", json!({"proposal_json":proposal})).await;
    assert_eq!(route["recommendation"], "escalate");
    assert_eq!(route["execution_authorized"], false);
    let workflow = good(
        &client,
        "validate_workflow_config",
        json!({"workflow_json":include_str!("../examples/project-workflow.json")}),
    )
    .await;
    assert_eq!(workflow["record_validation"], "passed");
    assert_eq!(workflow["role_count"], 14);
    assert_eq!(workflow["agents_dispatched"], false);
    assert_eq!(workflow["permissions_granted"], false);
    timeout(LIMIT, client.cancel()).await.unwrap().unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn bad_raw_records_arguments_and_unknown_tools_are_rejected_without_losing_session() {
    let client = client().await;
    for arguments in [
        json!({"plan_json":"{\"revision\":0,\"revision\":1}"}),
        json!({"plan_json":"{\"revision\":0,\"revis\\u0069on\":1}"}),
        json!({"plan_json":"{"}),
        json!({"plan_json":{}}),
        json!({"plan_json":" ".repeat(RECORD_LIMIT+1)}),
        json!({"plan_json":plan().to_string(),"execute":"evil"}),
        json!({}),
    ] {
        assert_eq!(
            call(&client, "validate_plan", arguments).await.is_error,
            Some(true)
        );
    }
    let proposal = r#"{"format_version":1,"available_routes":["qa"],"minimum_score":0.5,"minimum_margin":0.1,"proposals":[{"route":"qa","score":0.699999999999999999999999999999}]}"#;
    assert_eq!(
        call(&client, "route_proposal", json!({"proposal_json":proposal}))
            .await
            .is_error,
        Some(true)
    );
    assert_eq!(
        call(
            &client,
            "route_proposal",
            json!({"proposal_json":proposal.replace("0.699999999999999999999999999999","9e-1")})
        )
        .await
        .is_error,
        Some(true)
    );
    assert!(
        timeout(
            LIMIT,
            client.call_tool(CallToolRequestParams::new("execute"))
        )
        .await
        .unwrap()
        .is_err()
    );
    good(
        &client,
        "validate_plan",
        json!({"plan_json":plan().to_string()}),
    )
    .await;
    timeout(LIMIT, client.cancel()).await.unwrap().unwrap();
}

fn raw_child() -> Child {
    Command::new(env!("CARGO_BIN_EXE_orchestrator-mcp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap()
}
#[tokio::test(flavor = "current_thread")]
async fn malformed_duplicate_and_oversized_transport_frames_terminate_without_echo() {
    for raw in [
        b"{\n".to_vec(),
        b"{\"id\":1,\"id\":2}\n".to_vec(),
        vec![b'x'; FRAME_LIMIT + 1],
    ] {
        let mut child = raw_child();
        let mut stdin = child.stdin.take().unwrap();
        // Oversize causes intentional early close; broken pipe is an allowed result.
        let _ = timeout(LIMIT, stdin.write_all(&raw)).await.unwrap();
        drop(stdin);
        let mut stdout = child.stdout.take().unwrap();
        let mut output = Vec::new();
        timeout(LIMIT, stdout.read_to_end(&mut output))
            .await
            .unwrap()
            .unwrap();
        let status = timeout(LIMIT, child.wait()).await.unwrap().unwrap();
        assert!(!status.success());
        assert!(output.is_empty());
    }
}

#[tokio::test(flavor = "current_thread")]
async fn eof_unknown_process_arguments_and_incomplete_frame_timeout_exit_cleanly() {
    let mut child = raw_child();
    drop(child.stdin.take());
    assert!(
        !timeout(LIMIT, child.wait())
            .await
            .unwrap()
            .unwrap()
            .success()
    );
    let output = timeout(
        LIMIT,
        Command::new(env!("CARGO_BIN_EXE_orchestrator-mcp"))
            .arg("--listen")
            .output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let mut child = raw_child();
    // Child::wait closes child.stdin; hold the extracted pipe open to test the timer.
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"{").await.unwrap();
    let started = tokio::time::Instant::now();
    assert!(
        !timeout(FRAME_TIMEOUT + Duration::from_secs(4), child.wait())
            .await
            .unwrap()
            .unwrap()
            .success()
    );
    assert!(started.elapsed() >= FRAME_TIMEOUT - Duration::from_secs(1));
    assert!(started.elapsed() < FRAME_TIMEOUT + Duration::from_secs(3));
    drop(stdin);
}

#[tokio::test(flavor = "current_thread")]
async fn initialized_session_partial_frame_uses_fifteen_second_deadline() {
    let mut child = raw_child();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());

    let initialize = json!({
        "jsonrpc":"2.0", "id":1, "method":"initialize",
        "params":{"protocolVersion":"2025-06-18","capabilities":{},
            "clientInfo":{"name":"bounded-transport-test","version":"1"}}
    });
    stdin
        .write_all(format!("{}\n", initialize).as_bytes())
        .await
        .unwrap();
    let mut response = String::new();
    timeout(LIMIT, stdout.read_line(&mut response))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(serde_json::from_str::<Value>(&response).unwrap()["id"], 1);

    stdin
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
        .await
        .unwrap();
    stdin
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n")
        .await
        .unwrap();
    response.clear();
    timeout(LIMIT, stdout.read_line(&mut response))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(serde_json::from_str::<Value>(&response).unwrap()["id"], 2);

    stdin
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":3,")
        .await
        .unwrap();
    let started = tokio::time::Instant::now();
    let status = timeout(FRAME_TIMEOUT + Duration::from_secs(4), child.wait())
        .await
        .expect("incomplete initialized-session frame did not time out")
        .unwrap();
    assert!(!status.success());
    assert!(started.elapsed() >= FRAME_TIMEOUT - Duration::from_secs(1));
    assert!(started.elapsed() < FRAME_TIMEOUT + Duration::from_secs(3));
}

#[tokio::test(flavor = "current_thread")]
async fn initialized_sdk_session_exits_on_eof_and_has_no_protocol_stdout_noise() {
    let mut child = raw_child();
    let input = child.stdin.take().unwrap();
    let output = child.stdout.take().unwrap();
    // SDK parses stdout directly: any diagnostic noise causes this journey to fail.
    let client = timeout(
        LIMIT,
        ClientConfig::default()
            .with_protocol_version(ProtocolVersion::V_2025_06_18)
            .serve((output, input)),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(
        timeout(LIMIT, client.list_tools(None))
            .await
            .unwrap()
            .unwrap()
            .tools
            .len(),
        6
    );
    timeout(LIMIT, client.cancel()).await.unwrap().unwrap();
    assert!(
        timeout(LIMIT, child.wait())
            .await
            .unwrap()
            .unwrap()
            .success()
    );
}

#[tokio::test(flavor = "current_thread")]
async fn cancelled_receive_keeps_partial_bytes_and_original_deadline() {
    let (mut input, reader) = tokio::io::duplex(4096);
    let mut transport = BoundedTransport::new(reader, tokio::io::sink());
    input
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"")
        .await
        .unwrap();

    assert!(
        timeout(Duration::from_millis(100), transport.receive())
            .await
            .is_err()
    );
    input.write_all(b"}\n").await.unwrap();
    let message = timeout(Duration::from_secs(2), transport.receive())
        .await
        .unwrap();
    assert!(matches!(message, Some(JsonRpcMessage::Notification(_))));

    let (mut input, reader) = tokio::io::duplex(4096);
    let mut transport = BoundedTransport::new(reader, tokio::io::sink());
    input.write_all(b"{").await.unwrap();
    let started = tokio::time::Instant::now();
    assert!(
        timeout(Duration::from_secs(2), transport.receive())
            .await
            .is_err()
    );
    assert!(
        timeout(FRAME_TIMEOUT + Duration::from_secs(1), transport.receive())
            .await
            .unwrap()
            .is_none()
    );
    assert!(started.elapsed() >= FRAME_TIMEOUT - Duration::from_secs(1));
    assert!(started.elapsed() < FRAME_TIMEOUT + Duration::from_secs(2));
}

struct StalledWriter;
impl tokio::io::AsyncWrite for StalledWriter {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context<'_>, _: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Pending
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

fn response_with_text(text: String) -> rmcp::model::ServerJsonRpcMessage {
    JsonRpcMessage::response(
        ServerResult::CallToolResult(CallToolResult::success(vec![ContentBlock::text(text)])),
        RequestId::Number(1),
    )
}

#[tokio::test(flavor = "current_thread")]
async fn stalled_writer_times_out_and_output_frame_cap_is_enforced() {
    let mut stalled = BoundedTransport::new(tokio::io::empty(), StalledWriter);
    let started = tokio::time::Instant::now();
    let error = stalled
        .send(response_with_text("bounded output".into()))
        .await
        .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    assert!(started.elapsed() >= Duration::from_secs(4));

    let mut writable = BoundedTransport::new(tokio::io::empty(), tokio::io::sink());
    let error = writable
        .send(response_with_text("x".repeat(FRAME_LIMIT)))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("Output frame exceeds limit"));
}
