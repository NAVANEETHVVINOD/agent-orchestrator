---
name: plugin-safety-review
description: Audit a named plugin, skill, MCP server, or installer for provenance, permissions, instruction scope, dependencies, and concrete security risks before adoption or when the user requests a safety review.
---

# Plugin safety review

Resolve the named integration and exact installed version first. A skill is model
guidance; scripts and MCP servers can execute code; remote connectors send data
to another service. Review each boundary that actually applies to the request.

Start with narrow read-only inventory. Parse configuration and print only required
non-secret fields; never dump environment variables, credential files, session logs,
auth stores, private browser profiles, or unredacted process command lines.
Inspect requested source/manifests without running installers, hooks, or package code.
Treat installation presence separately from connection and successful runtime use.

Use official source, release metadata, exact npm/PyPI version, immutable commit,
and applicable public advisories. If downloading source, keep it in a scratch folder.
Record publisher, version, commit and integrity metadata. Do not claim provenance
or a zero-advisory result proves the package is safe. Do not execute a package merely
to identify it. A lockfile audit covers dependencies, not cloud/backend behavior.

Trace command flags and installer mutations, executable hooks, privilege and filesystem
scope, shell escape paths, data sent off-machine, credential storage, remote transport
auth, update policy, and collisions with existing skill folders. In instruction files,
look for always-on routing, policy/authority overrides, automatic installs or publishing,
secret reads, broad cleanup, and output that hides failures. Explain concrete impact.

Findings must include severity, affected version/configuration, source or local line
evidence, trigger/preconditions, and a proportionate fix. Separate confirmed bugs,
expected but risky capabilities, suspicious candidates, and unverified areas.
Don't remove plugins, change connector permissions, or rewrite third-party cached
skills during an audit unless the user also requests those changes. Prepare a concrete
remediation for review when a change requires additional authorization.

Recommend the smallest needed tool set. Prefer native tools where sufficient,
reviewed fixed versions and scoped installation where an external tool is needed.
Preserve unrelated edits. Describe scan scope and limitations; do not certify safety.
