---
name: decision-routing
description: Plan or evaluate bounded agent/tool routing with optional Jev/Laya-style decision models. Keep capability eligibility, permissions and release gates deterministic; validate scores and escalate uncertain or unavailable routes.
---

# Optional bounded decision routing

Use deterministic routing for clear rules or a small known task. A decision model is
useful when repeated bounded classifications have enough volume and measured benefit.
It proposes one of the eligible routes; the coordinator still owns interpretation,
authorization, verification and next actions. No model or score can approve installs,
data disclosure, permission changes, release readiness or an external write.

Read [decision-model contract](references/decision-models.md) before selecting a provider.
Research actual Jev/Laya publisher docs, exact versions/model artifacts, licenses and
compatibility through an available capability researcher. Resolve local/cloud data and
budget choices before adoption. This skill does not bundle weights, credentials or a
running decision service. Do not simulate a successful API call from vendor examples.

Derive eligible routes from the current task and actual host capabilities. Evaluate
typed outputs on held-out project examples, including ambiguous/unanswerable cases,
large option sets, changed tools and adversarial content. Scores are model estimates;
calibrate and choose escalation thresholds from measured error consequences.

The optional Rust `orchestrator route-proposal` CLI checks
a caller-supplied allowlist, finite scores and explicitly supplied minimum score/margin.
It recommends one declared route or escalation and never executes tools. Follow its
[decision-model contract](references/decision-models.md); a recommendation verifies neither capability availability nor consent.
All candidates supplied by the provider must be included. If scores are incomplete,
badly calibrated or stale, escalate and use ordinary planning/verified rules.
