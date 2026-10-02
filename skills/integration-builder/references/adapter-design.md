# Application adapter decision contract

| Available surface | Preferred implementation | Evidence before acceptance |
| --- | --- | --- |
| Official API/SDK | Small typed Rust client/CLI; expose selected operations through MCP when needed | Version/auth/scope verified; real request and error E2E; action receipts |
| Owned local application source | Add an explicit supported command or IPC/API boundary; then adapt it | Source authorization; compatibility contract; actual application tests |
| Supported local export/import | Rust format validator and narrowly scoped converter or importer | Documented format, backups, round-trip test, correct target and permissions |
| Supported UI automation only | Host-supported browser/accessibility automation with observable state checks | Actual supported automation surface, selector/state checks, user action scope and real UI journey |
| No accessible reliable surface | Research report and proposed upstream integration | Document limitation; do not manufacture a connector |

The adapter receives only operations authorized for its intended account/project.
Read access does not imply write access. Authentication and authorization live in
the backend/runtime as well as the instruction workflow. Classify data sensitivity
and keep logs sanitized. Validate inputs before service calls, bound output size,
time and concurrency, preserve cancellation, and distinguish accepted requests from
completed actions. Writes require stable operation identities and reconciliation
before retrying uncertain outcomes.

For remote MCP, validate intended audience and tool/resource access per request,
isolate user/tenant context and avoid using a single privileged account for all users.
For local stdio MCP, disclose the exact executable/arguments and filesystem/account
scope; stdout is protocol-only and diagnostics belong on stderr. Use the official
selected protocol/SDK version; no fabricated handshake or broad execute-command tool.

Publish source, installation/configuration steps and a compatibility matrix for the
actually tested version. Keep integration-specific services separate from the core
skills-only plugin. Adding a new service can change publisher review requirements;
check current official host documentation before claiming directory eligibility.
