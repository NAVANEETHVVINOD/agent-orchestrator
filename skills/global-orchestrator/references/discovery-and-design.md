# Requirements questions and project-specific design

Use this at project initiation or when a substantial feature leaves material choices
unanswered. Inspect prior user answers, project instructions, code and docs before
asking. Label confirmed facts, assumptions with consequences, and open decisions.

Ask in focused batches, usually 4–6 related questions when substantial discovery is
needed. Start with product scope, then design/data/security and delivery. Use the host's
question tool when available; respect its question limits. The coordinator presents
questions to the human after consolidating specialist input. An unanswered required
choice remains pending; elapsed time is not consent. Optional low-impact preferences
can use a stated assumption when the host allows it. Do not use a long survey verbatim
for a project that already supplies the answers.

## Question bank: select what materially applies

- Outcome: What problem and users does this serve? What proves success? MVP versus
  later phases? Which features are explicitly excluded? Existing product to extend?
- Application: Web, native mobile, desktop, API, CLI, library, data pipeline or embedded?
  Supported devices/platforms/browsers? Offline operation or unreliable connectivity?
- Users and permissions: Account types/roles? Authentication and recovery? Tenant or
  organization boundaries? Anonymous users? Admin and moderation responsibilities?
- Journeys: Primary actions, onboarding, navigation, search, payments or notifications?
  Failure, empty/loading, retry and cancellation states? What must happen across sessions?
- UI/UX: Required pages/screens and sections? Brand/design system? Accessibility goals?
  Responsive layouts? Language/locale/RTL? User-facing copy and available real content?
- Data: Entities and relationships? Validation/uniqueness? Ownership/access? Retention,
  deletion/export, uploads and backups? Existing database/schema/migration constraints?
- Backend/API: External providers, API contracts, background jobs, realtime updates,
  quotas, idempotency, observability and error handling? Existing integrations and auth?
- Technology: Required language/framework, existing tooling and deployment host?
  Team skill constraints, versions, package manager and license restrictions? Budget?
- Quality/security: Critical end-to-end journeys? Test data/environment? Sensitive data,
  abuse cases, threat boundaries or compliance requirements? Performance/reliability targets?
- Delivery: GitHub repo/branch strategy, CI access, required checks/review, environments,
  release/rollback expectations and production authority? Docs audience and maintenance?

Turn answers into testable requirements, not merely a prose summary. Record the source
of each material decision and its alternatives. Identify dependencies requiring user
accounts/credentials without requesting secrets in chat. Ask only the missing information
and route credentials through an approved auth mechanism when actual integration is needed.

## Design outputs

For UI applications, plan the sitemap/screens, section order, component inventory,
layout/grid/spacing, typography/colors, responsive rules and navigation. Make wireframes
show real intended hierarchy and behavior. Cover loading/empty/error/success states,
validation, disabled states, keyboard/focus and accessibility. Tie screens to user
journeys and backend contracts; avoid decorative screens without functioning flows.

Use existing relevant design skills and exposed design/browser tools. Read their entry
skills before use. Preview the resulting artifacts and request meaningful missing
preferences before implementing dependent choices. A wireframe may use labeled draft
content, but draft copy/fake actions cannot silently become shipping functionality.

Architecture records boundaries, state management, auth, endpoints/contracts, data
schema/indexes/migrations, background work, secrets/config, testing, deployment,
observability and rollback. The architect and UX planner reconcile conflicting contracts
through the coordinator before workers consume them.

## Adapt to the actual application and language

| Project type | Specific planning and E2E considerations |
| --- | --- |
| Web UI | Responsive routes/states, auth/server authorization, accessibility, client/server errors and real browser user journeys |
| Mobile | Platform navigation, permissions, offline/lifecycle behavior, device/emulator availability, build signing and device journeys |
| Desktop | Supported OS/window/input behavior, filesystem/IPC privileges, update process and native application journeys |
| API/backend | Contract/versioning, auth/object access, persistence/transactions, jobs and API-to-database/service journeys |
| CLI/library | Public interface/exit codes or package contracts, supported runtimes, install/build behavior and actual consumer workflows |
| Data/ML pipeline | Provenance, dataset/model limits, reproducibility, evaluation criteria, privacy, failure handling and full pipeline runs |

Respect the established stack in an existing project. For a new project, compare a
small set of candidates against confirmed needs and current official documentation;
do not choose a universal language, framework, database or testing tool. Keep runtime,
dependency and action versions deliberate and document tradeoffs. Do not force UI,
database, cloud services or authentication into a project that does not need them.
