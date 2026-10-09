# Changes

## Unreleased (0.4.0)

Adds a separately built local Rust stdio MCP facade over the deterministic planning
kernel. It exposes six bounded planning, workflow-validation and routing tools and does not
execute agents, access the filesystem or authorize external actions. Adds a declarative
JSON role/profile/skill mapping and quality-gate authoring schema with validation in the
Rust CLI/MCP. Windows debug/release tests and independent source/security review pass;
cross-platform hosted CI and ChatGPT web E2E remain pending. This is not yet a release or
a remote ChatGPT endpoint. See [MCP scope and remaining verification](MCP.md).

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
