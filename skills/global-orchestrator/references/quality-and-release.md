# Quality, security, evidence and GitHub gates

## Define verification before building

QA derives its plan from acceptance criteria and actual stack. Define critical
end-to-end journeys and the permitted local/test environment during discovery.
For an API/CLI/library, E2E means a real consumer workflow through the affected
boundaries; it does not automatically mean a browser test. Include relevant failure,
access-control, persistence and regression paths. UI projects also verify responsive,
keyboard/accessibility and error states. Production writes require explicit authority.

Use native project tooling and relevant installed test skills. Test fixtures/mocks may
support unit/integration tests but must not be represented as a live integration.
Record which dependencies were real versus simulated. A required live E2E blocked by
missing access remains blocked; do not replace it with a stub and declare it passed.

## Evidence and anti-hallucination review

qa_executor reports actual command, environment, tested source revision/fingerprint,
results and sanitized artifact references for each required check. Preserve failed,
skipped and unverified outcomes. verification_reviewer independently inspects behavior
and code. evidence_reviewer verifies API/library claims against repository evidence or
official docs, confirms tool/test outputs actually exist, and challenges fabricated
metrics, invented APIs, unsupported dependencies and unacknowledged assumptions.

Review production TODO implementations, dummy endpoints/data, disabled validation,
unimplemented handlers and fake-success paths. Distinguish deliberate test fixtures,
documented prototype scope and valid constants from harmful hardcoding. Do not ban
all literals or permit fake functionality merely because it looks complete. Ask about
missing business rules; do not invent account roles, prices, auth or third-party behavior.
No agent can guarantee freedom from hallucinations; require evidence and preserve limits.

## Security and vulnerability remediation

security_auditor reviews the actual application's trust boundaries: auth/session and
server-side authorization, cross-user/tenant data access, input/output handling,
uploads, secrets, database/storage policy, dependencies, network/SSRF and deployment
configuration as applicable. Use current official advisories and a stable relevant
security baseline, such as the researched OWASP ASVS release for web applications.
Scanners do not prove safe business logic or authorization; verify meaningful abuse cases.

Record each finding's affected code/version/config, evidence, severity, preconditions,
owner, fix and verification. Distinguish scanner candidates from confirmed reachability.
Prioritize severity and actual exposure; do not silently suppress findings or accept
release risk. Fix blockers, reproduce the original symptom, add a meaningful regression
check where warranted, rerun affected tests and review related paths/root causes.
Residual-risk decisions require the user's explicit acceptance and recorded rationale.
Keep all confirmed unresolved bugs/security issues visible; a backlog entry is not a fix.

## Iterative gates

The full release contract below applies to application or substantial-feature releases.
For a narrow documentation/configuration correction, use the checks relevant to that
change; the optional six-kind checker is unnecessary. Keep the user's explicit
verification requirements and the actual external-action authorization in either case.

| Gate | Pass criterion | If unmet |
| --- | --- | --- |
| Implementation acceptance | Requirements/design met; no fake production behavior; current docs | Return to owning requirement/design/code layer |
| Local pre-push | Actual required lint/type/build/unit/integration/E2E and independent security/code/evidence/docs reviews pass on current source; no release blockers | Fix/retest, or obtain the concrete missing test environment/input; do not push |
| Authorized GitHub push | Local gate satisfied and repository/branch/action authorized | Prepare concrete local work; await missing authority only for the external action |
| Hosted CI | Required checks actually successful on the current relevant head/test-merge/merge-group revision | Inspect failure evidence, fix locally, reverify and inspect the new pushed run |
| Merge/deploy | Current head, required checks, independent review, branch/environment rules, release/rollback and applicable authorization all satisfied | Preserve the gate and report the exact outstanding condition |

Apply the user's E2E-before-push requirement to application/feature code pushes.
Required checks skipped, missing, stale, cancelled or failing do not pass. For a check
that truly does not apply, document its reason and settle the verification contract
with the user rather than silently changing the gate. Do not re-request authorization
already supplied for the same external action and target in this session.

Source changes invalidate affected previous evidence. Record exact tested source and
inspect working tree/index/untracked changes before pushing. Evidence-only docs changes
can retain a source fingerprint only when the application does not consume those files
and the changes are reviewed; after push, hosted CI must still match the actual revision.

## CI/CD design

release_engineer generates/updates actual project workflows using confirmed runtime,
package manager/lockfile, test commands and deployment target. No universal fake workflow
or guessed GitHub repo. Mirror meaningful local checks in CI, including E2E fixtures
and sanitized retained artifacts. Existing workflows/branch rules are preserved and
improved within scope. Cloud features/permissions vary by repository; verify availability.

Default tokens to minimal permissions and grant scoped job needs. Pin external actions
and reusable workflows to verified full source SHAs; maintain updates deliberately.
Keep untrusted PR input out of shell interpolation and privileged execution. Do not
run untrusted PR code with secrets through pull_request_target/workflow_run. Use
appropriate isolated runners and short-lived deployment credentials where available.

Use a required final gate with always() and explicit required dependency outcomes;
reject absent, failed, cancelled, neutral or skipped required jobs. GitHub's built-in
required-check rules can accept neutral/skipped conclusions, so a green badge alone
does not prove E2E ran. Avoid path filters that suppress required workflows; handle
merge_group when merge queues require it. Verify actual remote settings rather than
claiming a committed YAML file establishes branch protection or deploy policy.

Hosted CI runs after the relevant GitHub event. Local checks/workflow emulation are
local evidence, not a prediction that hosted CI will pass. Do not claim merge/deploy
readiness before inspecting the real current run and reviewer state.

## Optional deterministic evidence checker

The optional Rust orchestrator project-gate CLI only reads the chosen Git repository and JSON report;
it runs read-only Git listing/revision commands, not tests, hooks, installers or pushes.
The checker snapshots tracked and nonignored untracked files and hashes their actual
content. docs/quality/ is reserved for generated evidence and excluded from that source
digest; keep application source/test code outside that directory. Symlinked files are
hashed as link text instead of following links outside the project. Generated/ignored
runtime dependencies are not part of this digest: lockfiles, environment and provenance
still need human/agent review. A file changed after tests invalidates the matching digest.

```text
orchestrator project-gate --project-root <repo> --snapshot
orchestrator project-gate --project-root <repo> --report docs/quality/release-evidence.json --stage pre-push
```

Populate report records from actual QA/reviewer results. Schema: format_version=1,
source_digest from --snapshot, blocking_findings=[], unresolved_decisions=[],
checks=[{id, kind, required, status, source_digest, evidence, evidence_sha256}].
Required kinds are build, e2e, code-review, security-review, evidence-review and docs-review.
Also record any required stack-specific unit/integration/lint/type checks. Evidence
paths must be sanitized files under docs/quality/ with matching hashes. For --stage
merge or deploy, ci.head_sha must equal the current Git HEAD and ci.checks must include
a required final-gate with status=success; every other required CI check must also be
success. This verifies reported CI records, not remote GitHub state or branch rules.

Only actual status=passed satisfies a local required check. The tool catches missing
records/artifacts, stale fingerprints and contradictory statuses; a fabricated but
internally consistent report can still pass. The primary/reviewer must inspect actual
commands, output and remote CI independently. A passing checker does not authorize
external actions, certify safety, or replace independent E2E and security testing.
