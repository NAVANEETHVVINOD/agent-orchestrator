# Project lifecycle and ownership

Apply the full lifecycle to a new application or substantial feature. For an existing
project, inspect actual code, docs, instructions and unrelated edits first. A narrow
bug fix can enter diagnosis/implementation directly using established requirements;
do not repeat a full product interview for every small edit.

The primary chat owns decisions and communication with the human. Use specialist
subagents in bounded waves within the current concurrency cap. Specialists produce
questions and evidence; the coordinator consolidates them rather than letting each
agent independently bombard the user or rewrite shared requirements.

| Phase | Primary owner / useful perspective | Output under docs/ | Transition criterion |
| --- | --- | --- | --- |
| Intake and research | Coordinator + project_planner | project/requirements.md, decisions/open-questions.md, research/sources.md | Intended outcome, users, scope and material constraints resolved |
| Design and architecture | solution_architect + ux_planner when applicable | architecture/overview.md, architecture/data-and-api.md, design/experience.md, design/wireframes.md | Stack/data/API/UX choices align with confirmed requirements; unresolved decisions explicitly block affected work |
| Delivery planning | Coordinator + project_planner | project/plan.md, project/task-ledger.md | Dependency-ordered tasks have owners, acceptance criteria and verification plan |
| Implementation | scoped_implementer / worker + domain skills | Updated affected design/contracts/decisions | Actual artifacts satisfy the assigned behavior; outputs ready for independent inspection |
| Quality review | verification_reviewer, evidence_reviewer, security_auditor | quality/review.md, quality/security.md, quality/evidence-review.md | Concrete findings triaged; unsupported claims removed; no unresolved release blockers |
| QA and E2E | qa_executor or scoped native fallback | quality/test-plan.md, quality/test-results.md and sanitized evidence | Actual required checks and critical end-to-end journeys pass on the current source |
| Correction and convergence | Owning worker + reviewers/QA + documentation_maintainer | Findings, decisions, task state and affected docs updated | Reproduction fixed, affected checks rerun, integrated behavior re-reviewed |
| GitHub and CI/CD | release_engineer + pr_auditor | delivery/ci-and-release.md | Local push gate met; push authorized; actual hosted CI checked before merge/deploy |
| Completion and learning | Coordinator + documentation_maintainer | project/status.md, quality/retrospective.md when useful | User outcome delivered with actual evidence, residual limits and current docs |

Create only useful documents for the actual scope; combine related records for a small
project. The table is a convention, not permission to generate empty scaffold files.
Existing project documents are adapted in place with links/indexes and preserved history.

Each substantial feature receives stable requirement IDs, task IDs and matching test
or review evidence. The ledger links requirement -> owned change -> verification ->
remaining finding -> next action. Review the integrated application as well as parts.

The optional [Rust planning kernel](PLANNING-KERNEL.md) can validate a versioned
project plan, propose bounded work waves, render its chart and apply reported
transitions. The coordinator supplies actual runtime capacity and capabilities,
dispatches through the host and verifies outputs. The kernel does not run the lifecycle.

Before production implementation, ask the human about unresolved choices that affect
product scope, security/data handling, UX or meaningful cost. Present a concrete
recommended option and alternatives. Draft bounded plans/wireframes while answers
are pending; do not implement dependent guesses or use fake services to imply completion.
A confirmed scope/plan need not be approved repeatedly. Explicit changes reopen only
the affected decisions and checks.

For every result, inspect the actual artifact/diff and acceptance evidence. Missing
output stays in review. A failure routes to the responsible layer: misunderstood
requirement, inconsistent design, wrong implementation, bad fixture or environment.
Document the root cause, fix, meaningful regression check, and revised decision.

Do not equate roleplay by one model with independent review. A separate native agent
or an independent human provides an independent review perspective. If unavailable,
record that limitation and do not claim an independent pass. Respect the live cap and
wait for or reuse agents rather than starting an unbounded tree.

Local checks and required E2E precede authorized GitHub pushes. Hosted CI can only be
observed after the relevant revision reaches GitHub. A failing CI run returns to the
correction loop; a new source change invalidates affected earlier evidence. Do not
bypass the gate because a workflow was skipped, a reviewer said 'looks good', or a
dashboard badge is green on an old revision.

No permanent background worker is installed. The coordinator performs this cycle
during active task work. Separate automations require an explicit scheduled request.

Research basis and immutable source snapshots are recorded in [RESEARCH.md](RESEARCH.md).
