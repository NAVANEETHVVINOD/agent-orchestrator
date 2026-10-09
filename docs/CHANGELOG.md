# Changes

## Unreleased (0.4.0)

Adds a separately built local Rust stdio MCP facade over the deterministic planning
kernel. It exposes six bounded planning, workflow-validation and routing tools and does not
execute agents, access the filesystem or authorize external actions. Adds a declarative
JSON role/profile/skill mapping and quality-gate authoring schema with validation in the
Rust CLI/MCP. Inbound frames are capped at 8 MiB with a 15-second completion deadline;
output writes remain capped at 5 seconds. Windows local debug/release tests and independent
review of the bounded transport/test correction passed. Earlier hosted Linux runs failed in
an MCP subprocess error path; the tests now serialize the process-heavy cases and add indexed
diagnostics. PR #7 and post-merge Linux/Windows validation, test and optimized CLI/MCP E2E
runs passed, as did both final gates ([PR run](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37902677375),
[merge-commit run](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37903287143)).
The current OSV exact-version scan found zero known advisories across 129 registry entries, and
license metadata was inventoried for all 129 entries. Independent review found no security or
reliability issue in the current local stdio source. ChatGPT web E2E and remote HTTP authentication
remain unverified. PR #8 merged the four-request bound and rolling
notification window as `e2ac715`; its exact PR-head run and merge-commit Linux/Windows runs passed
all checks and final-gate ([PR run](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37910731823),
[merge run](https://github.com/NAVANEETHVVINOD/agent-orchestrator/actions/runs/37912634478)).
This is not yet a release or remote ChatGPT endpoint.
See [MCP scope and remaining verification](MCP.md).

## 0.3.0 — 3 October 2026
Adds a stateless Rust task-planning kernel: dependency validation, bounded work-wave
proposals, scope conflicts, escaped charts and revision-checked review/correction
transitions. The host performs actual execution, identity and evidence verification.
CI adds Windows/Linux matrices and optimized real CLI E2E as a required final-gate
dependency. Product direction and unresolved hosted/UI/provider choices are explicit.
No new provider, installer, network listener or telemetry is introduced.

## 0.2.0 — 3 October 2026
Added capability-manager, project-knowledge and decision-routing skills; optional
read-only capability_researcher profile; project-scoped acquisition/creation lifecycle;
RAG design/evaluation and Jev/Laya primary-source research; a local bounded-route
record checker with uncertainty escalation. Fourteen skills and fourteen optional roles. Migrates local helpers to Rust; adds
source-grounded business workflows and application CLI/MCP adapter creation. Includes
CO_OP adaptation analysis and real consumer CLI tests.
No third-party installation, indexing, model inference or new remote service included.

## 0.1.0 — 2 October 2026
Initial portable package: nine workflow skills, optional native role profiles,
requirements/design/doc ownership, evidence-driven QA/security/correction loop,
source/evidence gate checker, local marketplace, icon, package CI and Apache-2.0.
No hosted MCP/A2A service, public repository or directory publication included.
