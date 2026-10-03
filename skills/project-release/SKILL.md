---
name: project-release
description: Prepare stack-specific CI/CD and assess current-source local E2E and actual hosted CI/review evidence before authorized delivery.
---

# Project Release

Use the actual confirmed language/runtime, lockfile, test commands, repo/branch and deployment target. Preserve existing workflows/rules; do not generate a universal fake CI pipeline.
Require current-source local checks and actual E2E acceptance journeys before application/feature code pushes. Missing/failed/skipped required checks block the push. Source changes invalidate affected evidence.
Use verified full action/reusable-workflow SHAs, minimal token permissions, safe untrusted input and appropriately isolated runners. Never execute untrusted PR code with secrets in privileged contexts.
Use an explicit final CI gate that rejects absent/failed/cancelled/neutral/skipped required jobs. Hosted CI is checked after the authorized push on the actual relevant head/test-merge/merge-group revision; local emulation is not hosted evidence.
Before merge/deploy, verify current head, actual checks, independent review, branch/environment rules, residual blockers and rollback. A YAML file does not establish remote branch protection.
No push, PR creation, remote settings change, merge, deployment or external messages without authority for the action/target. Preserve authorization already given. Keep current delivery instructions under docs/.
