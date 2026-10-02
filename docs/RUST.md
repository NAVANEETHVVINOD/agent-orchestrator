# Rust helpers

The package's local executable helpers are Rust, with pinned dependency versions and
Cargo.lock. The instruction skills require no server. Building the optional CLI needs
Rust 1.94.1 or a compatible newer stable toolchain, and Git for project-gate.

```text
cargo build --locked --release
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib --tests
```

Use the resulting target/release/orchestrator executable (or orchestrator.exe on
Windows). Add its directory to your own PATH if desired; the plugin does not do that.
These commands illustrate the CLI after it is available on PATH:

```text
orchestrator validate-package --root /path/to/agent-orchestrator
orchestrator project-gate --project-root /path/to/repository --snapshot
orchestrator project-gate --project-root /path/to/repository --report docs/quality/release-evidence.json --stage pre-push
orchestrator route-proposal --input /path/to/proposal.json
orchestrator validate-plan --input /path/to/plan.json
orchestrator next-wave --input /path/to/plan.json --runtime /path/to/runtime.json
orchestrator render-chart --input /path/to/plan.json
orchestrator apply-transition --input /path/to/plan.json --event /path/to/event.json
```

The [planning kernel](PLANNING-KERNEL.md) validates recorded dependencies and proposes
bounded work waves; transitions emit updated JSON to stdout, without writing files.
Validation and routing read files; project-gate also runs fixed read-only Git commands.
None installs tools, runs tests, changes source, starts a service or supplies external
action authorization. Failed validation exits with status 1 and sanitized JSON errors.
Record validation cannot prove the truth of submitted evidence. Package validation
checks structure, paths, metadata, profiles, links and privacy; official schema validation
is a separate build-time check, explicitly not claimed by the Rust helper.

Reports and route files are limited to 1 MiB; strict JSON rejects duplicate keys,
nonstandard constants, reserved internal number tags and nesting over 64 levels.
Routing accepts exact plain decimal scores with at most 28 fractional digits and no
exponent notation. Unsupported precision is rejected rather than rounded. Thresholds
are project inputs, never universal calibration. Source/evidence files are bounded to
256 MiB; Git output to 20 MiB, listing to 30,000 files and each command to 20 seconds.
Non-UTF-8 paths and submodules need separate review and are rejected. Evidence lives
under docs/quality/; keep executable source/tests outside that excluded directory.
Snapshot execute-mode semantics follow the host; regenerate evidence on a different OS.

The Rust migration replaces the package's Python checker/validator scripts and tests.
It does not rewrite the reference CO_OP desktop application. External researched
projects such as Laya may still have their own Python runtime; none is bundled here.
