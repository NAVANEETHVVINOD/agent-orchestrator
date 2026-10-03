# Documentation ownership and freshness

The documentation_maintainer owns the canonical narrative Markdown documents in
docs/. For a new project, create a useful docs/README.md index and a small relevant
set of requirements, architecture, design, delivery and quality documents. Existing
projects are updated without wholesale moves or overwriting unrelated edits.

## Location policy

Project narrative docs live under docs/. Keep mandatory loader/framework/community
entry files where their consumers require them: global/repository/nested AGENTS.md,
skill SKILL.md, required root README/security/license/contributing files, and provider
templates/configuration. Root entries can stay short and link to canonical docs.
Generated framework Markdown and third-party/vendor docs are not relocated automatically.
This exception preserves functioning discovery; it does not justify scattered new
project plans or duplicate competing sources of truth.

Suggested grouping, created only when relevant:

```text
docs/
  README.md
  project/requirements.md, plan.md, task-ledger.md, status.md
  decisions/open-questions.md and focused decision records
  architecture/overview.md, data-and-api.md
  design/experience.md, wireframes.md
  quality/test-plan.md, test-results.md, review.md, security.md, evidence-review.md
  delivery/ci-and-release.md
  research/sources.md
```

## Update workflow

Workers identify docs affected by code/design changes. The maintainer inspects the
actual diff/artifacts and updates only those documents, then checks relevant links,
paths and documented commands against the project. Architecture/API/schema changes,
UX behavior, setup, environment variables, migrations, test and release procedures
must not remain described as their old state. No credential values in docs.

Separate implemented facts from plans, examples and unresolved questions. Record real
test outcomes, environment limitations and source revision/fingerprint in quality
records. Never fill test counts, CI state, benchmark claims or completed checklists
from guesses. Documentation-only claims cannot clear a failing runtime test.

Maintain requirement/task/evidence links and concise decisions with rationale. Update
status and release notes when useful; do not rewrite every Markdown file on every edit.
If multiple agents want the same doc, the coordinator assigns one owner or serializes
edits. Large docs can link to focused pages instead of accumulating duplicated text.

Review the updated docs before local acceptance and after CI or release facts change.
If the maintainer spots a code/document disagreement, return it to the owning worker
or clarify the intended requirement; do not merely change the docs to hide a defect.

For this global skill itself, SKILL.md and its referenced resources remain in the
standard skill layout. Exported user guides belong under the portable bundle's docs/.
