---
name: project-security
description: Review application trust boundaries and concrete vulnerability findings, then verify scoped remediation and residual risks.
---

# Project Security

Inspect only the assigned system: auth/session, trusted-layer authorization, cross-user/tenant data, validation/output encoding, uploads, secrets, database/storage policy, dependencies/network and delivery configuration where relevant.
Use current official advisories and an applicable stable security standard. Trace reachability and abuse preconditions; scanners alone do not verify authorization or business logic.
Report affected source/version/config, evidence, severity, owner, narrow fix and reproduction/verification limits. Separate candidates, documented powerful capabilities and confirmed vulnerabilities. No blanket safety certification.
Route real findings to the responsible worker, reproduce the symptom, verify fixes and related root causes. Do not suppress alerts or accept residual release risk silently; user risk acceptance requires recorded rationale.
Do not read/print credentials, mutate live data, deploy or run intrusive external probes without scope and authority. Keep findings/current security docs under docs/.
