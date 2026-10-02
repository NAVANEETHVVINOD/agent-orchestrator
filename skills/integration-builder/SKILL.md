---
name: integration-builder
description: Assess applications without MCP or CLI interfaces and build a scoped Rust CLI or MCP adapter using supported APIs, SDKs, local formats or permitted UI automation, with real integration tests and clear access boundaries.
---

# Integration builder

Use when the user wants ChatGPT or another supported host to control an application
that has no suitable MCP server or CLI. Read
[references/adapter-design.md](references/adapter-design.md). Discover existing tools
first; reuse an adequate connector instead of duplicating it. Apply capability-manager
and a relevant MCP-builder skill when available, after reading their actual instructions.

Establish the exact application/version/platform, desired operations, account/data
scope, supported access method, host and success criteria. Ask about material gaps.
Research official API/SDK, automation, export/import and authentication documentation.
Inspect authorized local code when available. Distinguish a supported control interface
from a guessed endpoint, undocumented storage format or unsupported automation path.

Choose the smallest viable adapter: a Rust CLI for a supported operation contract,
a Rust MCP service exposing bounded typed tools, or a supported UI automation workflow.
Use official protocol SDK documentation and pinned reviewed dependencies. Check the
target host's actual transport and installation capabilities before deciding packaging.
A local stdio MCP server requires a host supporting local processes; a remote-only
host needs a separately hosted, authenticated service and appropriate data authorization.
Do not claim a plugin alone can start a service or control arbitrary desktop windows.

Assign an implementer explicit owned paths. Define typed operation schemas, read/write
classification, errors, authentication, limits, retry/idempotency behavior and audit
evidence. Keep credentials in approved host/OS storage. Avoid arbitrary shell tools,
unrestricted URLs/paths, hidden retries of writes and remote token passthrough.
For UI adapters, inspect real state and stable supported selectors, verify the actual
target after navigation, stop on ambiguity, and report changes that break automation.

Implement the confirmed scope, inspect source independently, review dependencies and
security, and run unit/integration checks plus actual target-application E2E in an
authorized test account/workspace. Mocks support development but cannot establish
live compatibility. Missing app/access/service leaves the live check unverified and
blocks claiming a working control integration. Keep a reversible smoke test and
rollback/unregistration instructions. Verify discovered MCP tools in the actual host.

The user can authorize adapter development once; do not request repeated approval
for routine edits and local tests. Resolve missing authority before login/account
changes, production writes, data upload, public hosting or externally visible actions.
If no reliable interface or permitted automation exists, return evidence and feasible
alternatives instead of a fake CLI, success stub or invented MCP connection.

This skill guides project-specific adapter creation. It does not bundle a universal
app controller, public listener or preconfigured remote service.
