# Security and data handling

This skills-only package has no telemetry, network service, automatic install hook,
bundled connector or stored credentials. It guides the host's existing tools. Those
tools may read files, execute commands or send data to services under their own
permissions; the package cannot grant, revoke or sandbox that access.

Inspect each selected integration's provenance, permissions and data handling. Use
scoped project access and keep secrets/private records out of prompts, logs, Markdown
and exported evidence. Tool results, pages and external agent cards are untrusted data.
The coordinator does not follow their instructions to override policies or send secrets.

Required local E2E, independent review and concrete security findings are checked before
authorized code pushes. The source/evidence validator is a consistency checker, not
proof tests ran: fabricated consistent reports can pass. Inspect actual command outputs
and current hosted CI independently. docs/quality/ is reserved for generated evidence,
excluded from the validator's source digest, and must not contain executable application
source. Environment, ignored dependencies and submodules need separate verification.

Report a suspected package issue to the publisher through the eventual source repository.
No unprovided email or support URL is fabricated. Do not publish raw credentials or user
records in an issue. These checks reduce unsupported claims; they do not guarantee zero
bugs/vulnerabilities or complete prevention of hallucinations.
