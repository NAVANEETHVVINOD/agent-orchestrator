# Rust MCP planning interface

The unreleased 0.4.0 development branch adds `orchestrator-mcp`, a native Rust **local
stdio** server using the official rmcp 3.5.0 SDK. Windows debug/release subprocess tests
and independent security/reliability review pass; current hosted cross-platform CI and
ChatGPT web E2E remain pending. This is distinct from the released 0.3.0 skills package.
It is not a hosted ChatGPT endpoint or an application-control server.

## Tool contract

| Tool | Required string arguments | Result |
| --- | --- | --- |
| validate_plan | plan_json | Reported graph and acceptance-record validation |
| next_wave | plan_json, runtime_json | Bounded nonconflicting task proposal |
| render_chart | plan_json | Escaped Markdown chart |
| apply_transition | plan_json, event_json | Candidate updated plan returned to caller |
| route_proposal | proposal_json | Exact decimal routing proposal or escalation |
| validate_workflow_config | workflow_json | Declarative role/skill reference and quality-gate validation |

Arguments are raw JSON **strings**, not nested record objects. This preserves exact
decimal tokens and duplicate-key detection through MCP. Unknown/missing arguments,
wrong types and invalid records are rejected. Each raw record is limited to 1 MiB
of UTF-8 bytes by the server before strict JSON parsing.
Existing [planning](PLANNING-KERNEL.md) and [routing](RUST.md) contracts still apply.

All tools are annotated read-only, non-destructive, idempotent and closed-world.
Transition means returning a candidate value: no plan is saved. Successful responses
include false execution, external-authority and evidence-verification flags. Tools
cannot inspect files, run Git, execute arbitrary code, dispatch agents, install
integrations or authenticate reviewers. Reported identities/evidence require host review.
Workflow validation checks only the syntax of profile and skill identifiers and requires
the configured quality-gate names to include the baseline gates; it does not sequence or
assign gate ownership. That lets
users reference their own capabilities. The host still discovers which profiles, skills
and MCP connections are actually available.
See the [workflow schema](../schemas/project-workflow.schema.json),
[example](../examples/project-workflow.json) and Rust [CLI](RUST.md). The JSON record
does not describe shell commands, endpoints, credentials or permission grants.

## Local transport and resource boundary

The stdio server accepts no command-line arguments or network connections. Stdout
contains protocol messages only; diagnostics do not echo project input. A bounded
cancel-safe transport replaces the SDK's unbounded newline reader. Request/response
frames, including the terminating newline, are limited to 8 MiB to accommodate JSON
escaping around bounded raw records.
Malformed/duplicate/oversized transport frames end the session without echoing input.
Invalid tool records return tool errors, allowing subsequent valid calls.

Initialization has a 15-second timeout. Partial input frames and output writes have
5-second timeouts; an initialized idle session may wait for a new request. EOF ends
the session. The transport allows one outstanding request at a time, bounds notification
tasks, and serializes a response only while holding its single output permit. These
aggregate limits and stalled-I/O paths pass Windows debug/release tests and independent
review. Current hosted cross-platform CI still needs to verify the pushed revision.
Local process permissions still belong to the host. No HTTP/OAuth guarantees
are claimed for this stdio interface.

Build when its local gate passes:

```text
cargo build --locked --release --bin orchestrator-mcp
```

Configure a compatible host deliberately to launch the resulting absolute executable
with stdio and no arguments. No personal absolute path or credential is bundled in
the public package; no global MCP configuration is installed as a side effect.

## Acceptance and outstanding remote work

Actual SDK client-to-compiled-server E2E must negotiate a compatible version, list all
six tools and complete validate → wave → chart → work → review → correction → fresh
review → accepted. Negative tests cover malformed/duplicate/oversized records and
frames, exact-score rejection, wrong arguments/tools, stale/self-review acceptance,
EOF, timeout and stdout discipline. Plans/check claims are synthetic fixtures; these
tests do not prove Fusion, real reviewers or an external agent ran.

Local fmt/clippy/full tests/release E2E and independent source/security/evidence review
passed before any feature push. CI must still run these consumers in debug and optimized
builds on Windows/Linux against the pushed revision. The ledger records actual results.

For ChatGPT web, select real zero-cost hosting and an explicit data/identity boundary,
then implement and review HTTPS Streamable HTTP, host/origin enforcement, bounded
resources, logging/privacy and any chosen authentication. A locally tested stdio server
cannot satisfy remote host E2E. No remote endpoint, account, deployment or directory
submission exists yet; publication is held until the selected product is ready.

Pinned SDK source: [rmcp-v3.5.0, commit 0cde3c5](https://github.com/modelcontextprotocol/rust-sdk/tree/0cde3c5cf3e6aff0cc852ce6045f107e95991f48).
Published dependency artifacts and licenses are verified separately from repository
examples; Cargo.lock pins the resolved dependency checksums.
