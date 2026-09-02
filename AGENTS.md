# Repository Instructions for Coding Agents

## Scope and authority

These instructions apply to `sts2-game-core`. Direct user instructions take precedence. The
target-local documents below define the detailed engineering policy:

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`docs/PRODUCT.md`](docs/PRODUCT.md)
- [`docs/CODING_STANDARDS.md`](docs/CODING_STANDARDS.md)
- [`docs/TESTING.md`](docs/TESTING.md)
- [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md)
- [`docs/LICENSING.md`](docs/LICENSING.md)
- [`docs/WORKFLOWS.md`](docs/WORKFLOWS.md)
- [`docs/POLICY_AS_CODE.md`](docs/POLICY_AS_CODE.md)
- [`RELEASING.md`](RELEASING.md)

## Target contract

This target owns host-independent domain state, actions, identity, validation, typed errors, and
deterministic policy. It is not an HTTP server, MCP implementation, gateway, process supervisor,
filesystem adapter, clock source, or concrete game-host integration. The target is currently in
codebase initialization; `crates/core` contains the target-owned pure semantic package. Keep future
changes within the same boundary and do not add unrelated product behavior or placeholder crates.

The future game mod may consume core types and validation. The gateway owns instance lifecycle and
routing, MCP owns protocol-to-gateway translation, and the harness owns orchestration, experiments,
and artifacts. The accepted sixth-target protocol repository may own only independently versioned,
language- and transport-neutral contracts with named consumers.

## Safety and provenance

- Preserve existing files and unrelated changes; do not initialize Git or alter sibling targets.
- Do not copy, vendor, transliterate, or cite another implementation's source as a product plan.
- Do not use or retain proprietary game files, saves, credentials, personal paths, or generated output.
- Record uncertainty as `unverified`; static structure and compilation never prove host/runtime behavior.
- Do not add empty product crates. Every future crate/module needs an owner, consumer, and test purpose.
- Keep dependencies directed inward. Core must not depend on HTTP, MCP, gateway, process, filesystem,
  or concrete host crates.
- Do not use `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!` for ordinary failures.
- Keep unsafe code forbidden in this target; host/FFI unsafe belongs at the mod boundary.
- Do not add a protocol path dependency in this wave. Protocol consumer mappings require a later
  contract gate with named consumers, versioning, provenance, and conformance evidence.

## Required validation

From this target root, run the policy command first and then the Rust gates:

```bash
cargo metadata --no-deps --format-version 1
cargo run --locked --package repo-policy -- --strict
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

Report exact commands, results, skipped checks, and unverified runtime boundaries. A green policy or
Rust run is not a compatibility, release, or merge claim.
