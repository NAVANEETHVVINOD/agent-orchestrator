# Business workflow contracts

| Task | Inputs to settle | Concrete deliverable | Verification |
| --- | --- | --- | --- |
| Business/project strategy | Decision, company facts, customers, market, constraints and horizon | Evidence-linked options, assumptions and executable next steps | Current primary sources; counterarguments; feasibility and owner review |
| Operations/client work | Client/project identity, contract, milestone, deadline and permitted communications | Work plan, proposal or communication draft with action ledger | Scope and access checks; approved recipient; actual tool receipt for any send |
| Finance | Currency, jurisdiction, period, accounting basis, records, formula and purpose | Reconciled analysis or invoice/budget draft with reproducible calculations | Exact units/rounding; finite numbers; source reconciliation; independent review |
| Legal | Jurisdiction/date, parties, intended use and governing documents | Sourced issue checklist or clearly labeled draft | Current authoritative law and contract references; unresolved assumptions; qualified review when needed |

Reuse existing capabilities before adding a service. A project can build a finance or
legal backend separately through discovery/design/implementation/QA; the plugin does
not create one merely by selecting this skill.

For an implemented backend, tenant and role checks must run server-side on reads and
writes. Approval is an explicit state transition on a specific action/artifact version,
with actor, timestamp and scope; repeat/stale approvals must not repeat side effects.
Invoice identities use tenant + contractual milestone + business operation identity,
not amount alone. Currency follows the confirmed agreement. Test equal-amount distinct
milestones, already-invoiced records, cross-tenant IDs and failed approvals. Delivery
adapters need destination-specific scope; no global fallback recipient for private data.

Keep calculation and source facts separate from judgment. Missing values stay missing;
do not fill financial records with plausible numbers. Exact constants and test fixtures
are acceptable when grounded and labeled. Model confidence cannot approve a contract
or certify accounting, compliance, safety or completion.

An ongoing reminder or automation requires a user-requested schedule and a supported
native automation/service with reviewed permissions, budget and failure handling.
The task ledger alone does not start a background worker.
