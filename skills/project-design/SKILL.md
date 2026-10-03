---
name: project-design
description: Plan confirmed project architecture, database/API boundaries and relevant UI/UX flows, pages, wireframes and layouts.
---

# Project Design

Start from confirmed requirements and actual repository constraints. Compare a small set of suitable technical choices using current official documentation; do not select a universal framework/database.
Plan component boundaries, auth and permissions, API contracts, schema/relationships/indexes/migrations, state, background work, observability, testing and deployment/rollback as applicable.
For a UI application, plan journeys, sitemap/screens, sections/layout/components, responsive behavior, typography/colors, navigation, loading/empty/error/success, validation and accessibility. Use relevant available design skills/tools and inspect resulting wireframes. Non-UI projects do not need artificial pages.
Reconcile UX, API and data contracts through the coordinator. Label draft content; do not let fake wireframe behavior silently become shipped functionality. Raise material missing decisions before dependent code.
Store canonical narrative design/architecture under docs/ and preserve mandatory loader locations. External design writes or provisioning need authorization for that action.
