//! Local, stateless planning MCP facade. No dispatch, persistence or authority.
use crate::{planning, routing, strict_json, workflow};
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
        JsonRpcMessage, ListToolsResult, PaginatedRequestParams, RequestId, ServerCapabilities,
        ServerConfig, Tool, ToolAnnotations,
    },
    service::{RequestContext, RxJsonRpcMessage, TxJsonRpcMessage},
    transport::Transport,
};
use serde_json::{Map, Value, json};
use std::{
    collections::HashSet,
    io,
    sync::{
        Arc, Mutex as SyncMutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    sync::Mutex,
    time::Instant,
};

pub const RECORD_LIMIT: usize = 1024 * 1024;
/// Bounds one wire frame, including its trailing newline. The transport permits
/// one accepted request at a time; a pipelined frame may be buffered but is
/// rejected before JSON parsing, bounding additional receive-side allocation.
pub const FRAME_LIMIT: usize = 8 * 1024 * 1024;
/// Gives local clients time to transmit a bounded, large request frame while
/// still limiting how long an incomplete stdio frame can hold the session.
pub const FRAME_TIMEOUT: Duration = Duration::from_secs(15);
/// Bounds output writes and transport shutdown independently of frame receive.
pub const IO_TIMEOUT: Duration = Duration::from_secs(5);
/// rmcp dispatches every received request into a task. Keep at most one request
/// awaiting its response write so pipelining cannot accumulate queued results.
/// Overload closes this stdio session; a new process/session can recover.
pub const OUTSTANDING_LIMIT: usize = 1;
pub const NOTIFICATION_LIMIT: usize = 64;

const DEFINITIONS: [(&str, &[&str], &str); 6] = [
    (
        "validate_plan",
        &["plan_json"],
        "Validate reported plan structure and acceptance records; does not verify evidence or dispatch agents.",
    ),
    (
        "next_wave",
        &["plan_json", "runtime_json"],
        "Propose a bounded task wave from reported slots, dependencies and capabilities; no reservation or dispatch.",
    ),
    (
        "render_chart",
        &["plan_json"],
        "Render an escaped Markdown task chart from reported state; does not modify any file or UI.",
    ),
    (
        "apply_transition",
        &["plan_json", "event_json"],
        "Return a candidate plan after a guarded transition; no persistence, authentication or authorization.",
    ),
    (
        "route_proposal",
        &["proposal_json"],
        "Evaluate exact bounded route scores; no model inference, tool execution or authorization.",
    ),
    (
        "validate_workflow_config",
        &["workflow_json"],
        "Validate declarative role/profile/skill references and required quality gates; does not discover host availability, grant permissions, install tools or dispatch agents.",
    ),
];

fn tools() -> Vec<Tool> {
    DEFINITIONS.iter().map(|(name, fields, description)| {
        let properties: Map<String, Value> = fields.iter().map(|field| ((*field).into(), json!({
            "type":"string",
            "description":"Raw JSON text, limited by the server to 1 MiB UTF-8 bytes. Duplicate keys, malformed JSON and excessive depth are rejected."
        }))).collect();
        Tool::new(*name, *description, json!({"type":"object", "properties":properties,
            "required":fields, "additionalProperties":false}).as_object().unwrap().clone())
            .with_annotations(ToolAnnotations::new().read_only(true).destructive(false).idempotent(true).open_world(false))
    }).collect()
}

fn evaluate(name: &str, arguments: Option<Map<String, Value>>) -> crate::Result<Value> {
    let (_, fields, _) = DEFINITIONS
        .iter()
        .find(|(known, _, _)| *known == name)
        .ok_or("Unknown tool")?;
    let arguments = arguments.ok_or("Required arguments missing")?;
    if arguments.len() != fields.len()
        || arguments.keys().any(|key| !fields.contains(&key.as_str()))
    {
        return Err("Unknown or missing tool argument".into());
    }
    let record = |field: &str| -> crate::Result<Value> {
        let raw = arguments
            .get(field)
            .and_then(Value::as_str)
            .ok_or("Argument must be raw JSON text")?;
        if raw.len() > RECORD_LIMIT {
            return Err("JSON record exceeds 1 MiB".into());
        }
        strict_json::parse(raw.as_bytes())
    };
    let mut result = match name {
        "validate_plan" => planning::validate(&record("plan_json")?)?,
        "next_wave" => planning::next_wave(&record("plan_json")?, &record("runtime_json")?)?,
        "render_chart" => json!({"markdown":planning::render_chart(&record("plan_json")?)?}),
        "apply_transition" => planning::transition(&record("plan_json")?, &record("event_json")?)?,
        "route_proposal" => routing::evaluate(&record("proposal_json")?)?,
        "validate_workflow_config" => workflow::validate(&record("workflow_json")?)?,
        _ => return Err("Unknown tool".into()),
    };
    // These describe actual facade behavior, not trust in the caller's records.
    result["execution_performed"] = json!(false);
    result["external_action_authorized"] = json!(false);
    result["reported_evidence_verified"] = json!(false);
    Ok(result)
}

#[derive(Clone, Default)]
pub struct PlanningServer;
impl ServerHandler for PlanningServer {
    fn get_info(&self) -> ServerConfig {
        let mut config = ServerConfig::default();
        config.capabilities = ServerCapabilities::builder().enable_tools().build();
        config.server_info = Implementation::new(
            "agent-orchestrator-local-planning",
            env!("CARGO_PKG_VERSION"),
        );
        config.instructions = Some("Stateless local planning only. JSON string fields preserve exact decimals and duplicate detection. The host must inspect real artifacts, authenticate actors, reserve work and authorize connected tools. Records are not proof. No filesystem, network, persistent state or agent dispatch tools.".into());
        config
    }
    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        if request.and_then(|r| r.cursor).is_some() {
            return Err(ErrorData::invalid_params(
                "Pagination cursor is unsupported",
                None,
            ));
        }
        Ok(ListToolsResult {
            tools: tools(),
            ..Default::default()
        })
    }
    fn get_tool(&self, name: &str) -> Option<Tool> {
        tools().into_iter().find(|tool| tool.name == name)
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if !DEFINITIONS.iter().any(|(name, _, _)| *name == request.name) {
            return Err(ErrorData::invalid_params("Unknown tool", None));
        }
        let result = match evaluate(&request.name, request.arguments) {
            Ok(value) => CallToolResult::structured(value),
            Err(reason) => CallToolResult::error(vec![ContentBlock::text(reason)]),
        };
        Ok(result.into())
    }
}

/// Cancel-safe bounded newline framing, replacing the SDK's unbounded read_until.
/// Invalid/duplicate/oversized frames terminate this local session without echoing input.
pub struct BoundedTransport<R, W> {
    reader: BufReader<R>,
    writer: Arc<Mutex<W>>,
    partial: Vec<u8>,
    deadline: Option<Instant>,
    outstanding: Arc<SyncMutex<HashSet<RequestId>>>,
    notifications: usize,
    failed: Arc<AtomicBool>,
}
impl<R: AsyncRead, W> BoundedTransport<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer: Arc::new(Mutex::new(writer)),
            partial: Vec::new(),
            deadline: None,
            outstanding: Arc::new(SyncMutex::new(HashSet::new())),
            notifications: 0,
            failed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn failure_flag(&self) -> Arc<AtomicBool> {
        self.failed.clone()
    }

    fn fail(&self) -> Option<RxJsonRpcMessage<RoleServer>> {
        self.failed.store(true, Ordering::Release);
        None
    }
}

struct LimitedBytes(Vec<u8>);
impl io::Write for LimitedBytes {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > FRAME_LIMIT.saturating_sub(self.0.len()) {
            return Err(io::Error::other("Output frame exceeds limit"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn serialize_frame(item: &TxJsonRpcMessage<RoleServer>) -> io::Result<Vec<u8>> {
    let mut bytes = LimitedBytes(Vec::new());
    serde_json::to_writer(&mut bytes, item).map_err(io::Error::other)?;
    // FRAME_LIMIT covers the complete wire frame, including its newline.
    if bytes.0.len() >= FRAME_LIMIT {
        return Err(io::Error::other("Output frame exceeds limit"));
    }
    bytes.0.push(b'\n');
    Ok(bytes.0)
}

impl<R, W> Transport<RoleServer> for BoundedTransport<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send + 'static,
{
    type Error = io::Error;
    fn send(
        &mut self,
        item: TxJsonRpcMessage<RoleServer>,
    ) -> impl Future<Output = io::Result<()>> + Send + 'static {
        let writer = self.writer.clone();
        let outstanding = self.outstanding.clone();
        let failed = self.failed.clone();
        let response_id = match &item {
            JsonRpcMessage::Response(response) => Some(response.id.clone()),
            JsonRpcMessage::Error(error) => error.id.clone(),
            _ => None,
        };
        async move {
            let result = tokio::time::timeout(IO_TIMEOUT, async {
                // Serialize only after acquiring the single output permit.
                let mut writer = writer.lock().await;
                let frame = serialize_frame(&item)?;
                writer.write_all(&frame).await?;
                writer.flush().await
            })
            .await
            .unwrap_or_else(|_| Err(io::Error::new(io::ErrorKind::TimedOut, "Output timeout")));
            if let Some(id) = response_id {
                outstanding
                    .lock()
                    .map_err(|_| io::Error::other("Transport unavailable"))?
                    .remove(&id);
            }
            if result.is_err() {
                failed.store(true, Ordering::Release);
            }
            result
        }
    }
    async fn receive(&mut self) -> Option<RxJsonRpcMessage<RoleServer>> {
        loop {
            let available = match self.deadline {
                Some(deadline) => {
                    match tokio::time::timeout_at(deadline, self.reader.fill_buf()).await {
                        Ok(Ok(available)) => available,
                        Ok(Err(_)) | Err(_) => return self.fail(),
                    }
                }
                None => match self.reader.fill_buf().await {
                    Ok(available) => available,
                    Err(_) => return self.fail(),
                },
            };
            if available.is_empty() {
                return if self.partial.is_empty() {
                    None
                } else {
                    self.fail()
                };
            }
            let count = available
                .iter()
                .position(|byte| *byte == b'\n')
                .map_or(available.len(), |n| n + 1);
            if count > FRAME_LIMIT.saturating_sub(self.partial.len()) {
                return self.fail();
            }
            let complete = available[count - 1] == b'\n';
            self.partial.extend_from_slice(&available[..count]);
            self.reader.consume(count);
            if complete {
                // Do not build a second parsed request while a prior reply is
                // outstanding. The raw receive buffer is still frame-bounded.
                let has_outstanding = match self.outstanding.lock() {
                    Ok(outstanding) => !outstanding.is_empty(),
                    Err(_) => return self.fail(),
                };
                if has_outstanding {
                    return self.fail();
                }
                let parsed = match strict_json::parse(&self.partial) {
                    Ok(parsed) => parsed,
                    Err(_) => return self.fail(),
                };
                let message: RxJsonRpcMessage<RoleServer> = match serde_json::from_value(parsed) {
                    Ok(message) => message,
                    Err(_) => return self.fail(),
                };
                match &message {
                    JsonRpcMessage::Request(request) => {
                        if request.id.to_string().len() > 128 {
                            return self.fail();
                        }
                        let accepted = match self.outstanding.lock() {
                            Ok(mut outstanding) => {
                                outstanding.len() < OUTSTANDING_LIMIT
                                    && outstanding.insert(request.id.clone())
                            }
                            Err(_) => return self.fail(),
                        };
                        if !accepted {
                            return self.fail();
                        }
                    }
                    JsonRpcMessage::Notification(_) => {
                        // This server makes no peer requests. Bound all notification tasks
                        // too, including cancellations/progress sent without a request.
                        self.notifications += 1;
                        if self.notifications > NOTIFICATION_LIMIT || self.partial.len() > 4096 {
                            return self.fail();
                        }
                    }
                    JsonRpcMessage::Response(_) | JsonRpcMessage::Error(_) => return self.fail(),
                }
                self.partial.clear();
                self.deadline = None;
                return Some(message);
            }
            self.deadline
                .get_or_insert_with(|| Instant::now() + FRAME_TIMEOUT);
        }
    }
    async fn close(&mut self) -> io::Result<()> {
        match self.outstanding.lock() {
            Ok(mut outstanding) => outstanding.clear(),
            Err(_) => {
                self.failed.store(true, Ordering::Release);
                return Err(io::Error::other("Transport unavailable"));
            }
        }
        self.partial.clear();
        self.deadline = None;
        let result = tokio::time::timeout(IO_TIMEOUT, async {
            self.writer.lock().await.shutdown().await
        })
        .await;
        let result = match result {
            Ok(result) => result,
            Err(_) => {
                self.failed.store(true, Ordering::Release);
                return Err(io::Error::other("Close timeout"));
            }
        };
        if result.is_err() {
            self.failed.store(true, Ordering::Release);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    fn response_with_text(text: String) -> TxJsonRpcMessage<RoleServer> {
        JsonRpcMessage::response(
            rmcp::model::ServerResult::CallToolResult(CallToolResult::success(vec![
                ContentBlock::text(text),
            ])),
            rmcp::model::RequestId::Number(1),
        )
    }

    #[test]
    fn output_frame_cap_includes_the_newline_at_the_exact_boundary() {
        let base = serde_json::to_vec(&response_with_text(String::new())).unwrap();
        let text_len = FRAME_LIMIT - 1 - base.len();
        let exact = serialize_frame(&response_with_text("x".repeat(text_len))).unwrap();
        assert_eq!(exact.len(), FRAME_LIMIT);
        assert_eq!(exact.last(), Some(&b'\n'));

        let over = serialize_frame(&response_with_text("x".repeat(text_len + 1))).unwrap_err();
        assert!(over.to_string().contains("Output frame exceeds limit"));
    }

    #[tokio::test]
    async fn pipelined_request_overload_closes_and_clears_the_pending_slot() {
        let (mut input, reader) = tokio::io::duplex(4096);
        let mut transport = BoundedTransport::new(reader, tokio::io::sink());
        input
            .write_all(
                b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\",\"params\":{}}\n{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n",
            )
            .await
            .unwrap();

        let first = transport.receive().await.unwrap();
        assert!(matches!(first, JsonRpcMessage::Request(_)));
        assert_eq!(
            transport
                .outstanding
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .len(),
            OUTSTANDING_LIMIT
        );
        assert!(transport.receive().await.is_none());
        assert!(transport.partial.len() <= FRAME_LIMIT);
        assert!(
            transport
                .outstanding
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .len()
                <= OUTSTANDING_LIMIT
        );

        transport.close().await.unwrap();
        assert!(transport.partial.is_empty());
        assert!(
            transport
                .outstanding
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .is_empty()
        );

        // The overloaded session has no state carried into a fresh transport.
        let (mut next_input, next_reader) = tokio::io::duplex(4096);
        let mut next_session = BoundedTransport::new(next_reader, tokio::io::sink());
        next_input
            .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\",\"params\":{}}\n")
            .await
            .unwrap();
        assert!(matches!(
            next_session.receive().await,
            Some(JsonRpcMessage::Request(_))
        ));
        next_session.close().await.unwrap();
    }
}
