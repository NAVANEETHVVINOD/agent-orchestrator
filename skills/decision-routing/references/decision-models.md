# Decision models as an optional proposal layer

Jev and Laya are researched separately: a hosted vendor API and a separately published
open-weight implementation are not automatically interchangeable. Verify current SDK,
request/response behavior, model revision/license, option/context limits and truncation,
language/hardware needs and privacy. Model code and weight licenses/provenance both matter.
Use publisher docs/source/model cards and project measurements, not third-party speed
claims. A wire-compatible server does not prove matching semantics or authentication.

Research baseline, 2 October 2026: [TypeSafe API](https://docs.typesafe.ai/api),
[versioned Jev models](https://docs.typesafe.ai/models) and
[confidence semantics](https://docs.typesafe.ai/confidence) describe the hosted API.
The current documented version is jev-1.13.0; use a reviewed version instead of a
moving alias. Hosted input processing needs a separate privacy/data decision.

Laya's researched, pinned repository snapshot is
[4aa6761be8173de4ce6d92c31b3e40b6eaf59a7c](https://github.com/NandhaKishorM/laya/tree/4aa6761be8173de4ce6d92c31b3e40b6eaf59a7c),
and the official convaiinnovations/laya model revision is
[55cf4c4ebb4ebe31b2550e8bdf3bd21b99753851](https://huggingface.co/convaiinnovations/laya/tree/55cf4c4ebb4ebe31b2550e8bdf3bd21b99753851).
Code and model metadata use Apache-2.0. Publisher documentation warns of overconfidence,
option-token constraints and weak zero-shot typed decisions. CPU/GPU compatibility and
context limits depend on the selected checkpoint/runtime; actual hardware and task
performance were not tested here. Do not transfer Jev thresholds to Laya unchanged.

These are source-review references, not installed dependencies or performance guarantees.

## Controller and evidence

The coordinator defines a bounded decision and eligible options from actual capabilities,
scope and dependencies. Deterministic controls remove unavailable/unauthorized actions
before a model is asked to route; returned IDs are checked against that current list.
Probability never supplies authority or certifies requirements, code/security or test
results. The ordinary review/QA/E2E/CI gates still apply after a route is selected.

Start with rules/current coordinator as a baseline. Only adopt a model when held-out
project data demonstrates useful accuracy, calibration, latency and cost. Test unknown
intent, ties/low margin, abstention, multilingual inputs, option sets/context near actual
limits, malformed scores, stale availability and provider failures. Exclude secrets from
routing inputs; a hosted provider receives those inputs and needs authorized data scope.

Use explicit project-calibrated thresholds, retain sanitized decision evidence and version
the provider/model, option set and decision rubric. Escalate low-confidence, ambiguous,
invalid or unavailable results to the coordinator. Do not invent probability when a
provider only returns an ordinal score; confirm semantics or use the appropriate rule.
Failures should disable that adapter rather than expand access or switch providers
with private data without authorization. Model updates reopen relevant evaluation.

## Local proposal-checker contract

Build the optional Rust CLI from the plugin source (Cargo build --locked --release). Run with a sanitized JSON file after making the executable available:

```text
orchestrator route-proposal --input <proposal.json>
```

Fields: format_version=1; available_routes=[unique nonempty string IDs];
proposals=[{route, score}] with unique IDs and finite numeric scores in [0,1];
minimum_score and minimum_margin as explicit finite numbers in [0,1]. Use exact plain
decimals with at most 28 fractional digits; exponent notation and unsupported precision
are rejected instead of rounded. Every declared
available route must have a proposal, with no additional IDs. Thresholds are required
inputs chosen from real evaluation, not universal built-in calibration.

The utility rejects malformed, duplicate-key, oversized or inconsistent input. A score
below minimum_score, a tied top choice or insufficient top-two margin recommends
escalation. A single available route needs no ambiguity margin. Output always reports
execution_authorized=false, capability_availability_verified=false and
model_calibration_verified=false. Caller-supplied availability and scores are untrusted
records; independently inspect their origin and freshness. The script performs no
network call, installation, service launch, credential access or tool execution.
