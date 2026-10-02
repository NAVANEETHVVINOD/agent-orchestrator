# Local planning kernel

The optional Rust CLI operates on bounded JSON records supplied by the coordinator.
It performs no agent dispatch, tool calls, installation, network access or file writes.
Store a project's plan under its own docs/ folder. Save returned plans deliberately;
the CLI does not provide durable storage, locking or authenticated event history.

```shell
orchestrator validate-plan --input docs/examples/plan.json
orchestrator next-wave --input docs/examples/plan.json --runtime docs/examples/runtime.json
orchestrator render-chart --input docs/examples/plan.json
orchestrator apply-transition --input docs/examples/plan.json --event docs/examples/start.json
```

The bundled [plan](examples/plan.json), [runtime](examples/runtime.json) and
[start event](examples/start.json) are synthetic examples, not observations of an
installed capability, real review or successful application test.

## Required records

Every input is strict JSON, at most 1 MiB. Duplicate decoded keys and nonstandard
number constants are rejected. Identities are ASCII slugs beginning with a letter
or digit, followed by letters, digits, dash, underscore or dot; maximum 128 bytes.
Additional metadata may be preserved but is not interpreted or authorized.

A plan requires format_version (1), project_id, unsigned revision, filesystem_case
(sensitive or insensitive), decisions, capabilities and tasks. Decisions contain
id and status (open or resolved). Capabilities contain id and kind (native-agent,
skill, plugin, mcp or a2a). These declarations do not establish connection or permission.

Every task requires id, title (nonempty, at most 256 bytes), owner, kind, status,
depends_on, capabilities, requires_decisions, reads, writes, source_revision,
required_checks, checks, findings, artifacts and review. Arrays must be explicit,
including empty arrays. Review must explicitly be null or an object. A check contains
id, status (passed, failed, skipped or unverified) and source_revision. A review contains
actor, source_revision and status (passed or failed). Findings are nonempty text strings.

Paths use forward slashes, are concrete project-relative files or directories and
exclude traversal, absolute paths, wildcard syntax, backslashes, colons, controls,
empty segments and trailing dots/spaces. Ownership comparison uses path components
and explicit case semantics. It does not resolve symlinks, hard links, mount aliases
or authenticate a real filesystem; the host must inspect actual paths and sandbox
access before dispatch. Read/read overlap is allowed; write/write and read/write
overlap with active or selected work prevents concurrent selection.

Limits: 1,000 tasks and items per array, 10,000 dependency edges and check records,
32 combined read/write scopes per task and 4,096 scopes per plan. Unknown references,
duplicate IDs, self-dependencies and cycles are rejected. Accepted tasks require
accepted dependencies, resolved decisions, no findings, all required checks passed
for their source_revision and a passed current review with actor different from owner.
This validates reported claims, not the truth or identity behind those claims.

## Wave proposals

Runtime requires format_version (1), matching project_id and revision,
free_worker_slots (0–32), available_capabilities and active_tasks. Slots mean free
child-worker slots, excluding the coordinator; the host supplies actual live capacity.
Every running task must be listed active. Active tasks must be running or in review;
review tasks need only be active when their scope is still being used.

next-wave considers planned and needs-fix tasks in stable ID order. Dependencies,
decisions, capabilities, capacity and conflicting scopes constrain selection.
selected_tasks contains proposed IDs; tasks explains waiting reasons. No IDs execute.
The host must recheck current state and its actual authorization before dispatching.

## Events and correction cycle

An event requires format_version (1), project_id, expected_revision, task_id, actor,
from and to. A stale revision or state fails. Allowed edges are:

| From | To |
| --- | --- |
| planned | running, blocked, cancelled |
| running | review, needs-fix, blocked |
| review | accepted, needs-fix, blocked |
| needs-fix | running, blocked, cancelled |
| blocked | planned, cancelled |
| accepted | needs-fix |

An event may replace checks, review, findings, artifacts or source_revision.
Entering review requires the recorded owner as actor. Acceptance requires the actor
to match the passed independent review. Entering running requires ready prerequisites;
it does not independently reserve runtime slots or validate actual execution permission.
Source changes, every transition into needs-fix, and needs-fix to running clear checks
and review; supplying replacement checks or review in these events is rejected.
Record correction findings in findings and keep the actual failed reports in project
evidence. Reopen accepted downstream results before reopening a prerequisite, preventing
silently stale accepted descendants. Reopened work requires fresh QA/review records
even if the source label is unchanged.

apply-transition returns an envelope with plan, new revision and from/to. Subsequent
commands consume the envelope's plan field, not the whole envelope. Invalid events
fail without writing. The caller is responsible for atomic persistence and concurrent
updates; expected_revision alone supplies no lock.

All JSON successes report execution_performed, external_action_authorized and
reported_evidence_verified as false. render-chart emits escaped Markdown with a
reported-state notice. Separate actual QA, source freshness, reviewer identity,
security review, GitHub checks and release authorization remain required. See
[release evidence](RUST.md) and [product scope](PRODUCT.md).
