# Project roles

The main agent coordinates requirements, decomposition, integration and next actions.
These fourteen optional Codex profiles are under assets/codex-agent-profiles/. They
are not auto-installed, and other hosts may expose different role systems. Reuse
equivalent existing roles before adding profiles; preserve same-name custom files.

| Role/profile | Responsibility and expected output | Default permission |
| --- | --- | --- |
| project_planner | Consolidated questions, requirements, acceptance criteria and dependency plan | Read-only |
| solution_architect | Stack/data/API decisions, contracts, tradeoffs and architecture evidence | Read-only |
| ux_planner | User journeys, pages, layouts, wireframes and applicable accessibility needs | Read-only |
| evidence_explorer | Narrow codebase questions with source evidence | Read-only |
| business_analyst | Scoped business/operations/legal/finance analysis with sources and assumptions | Read-only |
| capability_researcher | Primary-source tool/model research, comparison, source pins and proposed validation | Read-only |
| scoped_implementer | Owned-file implementation, changed files and initial verification | Workspace write |
| verification_reviewer | Independent correctness/regression findings from actual source | Read-only |
| qa_executor | Actual scoped tests, E2E journeys and passed/failed/skipped/unverified evidence | Workspace write |
| security_auditor | Auth/data/secret/dependency boundaries and confirmed exposure | Read-only |
| evidence_reviewer | Unsupported assumptions, invented APIs/results, harmful hardcoding and fake behavior | Read-only |
| documentation_maintainer | Current factual docs, decisions, links and quality records | Workspace write |
| release_engineer | Confirmed-stack CI/CD, current-source gates and release/rollback evidence | Workspace write |
| pr_auditor | Current PR head, checks, review and merge-gate evidence | Read-only |

Read-only planners return artifacts to the coordinator for integration. A writer owns
only explicitly assigned paths. Give QA permitted fixtures/services and release work
the intended target and actions; a profile grants no external-action authorization.
Use bounded waves within host concurrency limits, with disjoint parallel ownership.
Built-in explorer/worker roles can provide scoped fallbacks. If independent review or
execution is unavailable, report that limit instead of simulating a pass.

Profiles inherit the parent model. Sandbox defaults remain subject to runtime policies
and overrides; instructions do not enforce operating-system access. Other users'
existing roles, connectors and repository rules retain their own constraints.

Capability creation uses the existing scoped_implementer/worker with explicit paths;
the researcher does not install its recommendations. Use the primary/default agent
with equivalent read-only instructions for public research when the custom role is
unavailable; built-in explorer is reserved for codebase questions.
