# sts2-game-core

Status: active Git-backed target repository. It contains one small, pure Rust core package plus its
target-local policy tool.

## Owner and consumers

The core maintainers own host-independent STS2 domain state, action descriptions, identity and
generation values, validation, typed domain errors, and deterministic policy. The
`sts2-game-mod` target is the primary future consumer at the host boundary. The initialized
`sts2-game-core` package is under `crates/core`; its public seam is covered by deterministic tests.
No sibling target may reach into core internals.

The accepted sixth target supplies a release-like `poc-v1` artifact. This repository verifies a local
copy of that artifact as data; it does not import protocol implementation modules or create a sibling
path dependency.

## Initial semantic seam

The package provides typed `Identity`, `Generation`, `Phase`, `State`, `Action`, and `Request` values,
pure `validate` logic, and checked application of an accepted proposal. The POC models one actor-owned
bounded resource: `use_budget` changes available units, generation, and settled-effect count exactly
once; zero units is rejected without a state change. Acceptance remains semantic evidence, not host
execution.

## Non-goals and boundaries

Core does not own HTTP, MCP framing or tools, gateway lifecycle/routing, process management, filesystem
or persistence behavior, clocks, model/provider behavior, concrete host objects, loader/FFI behavior,
or authoritative host mutation. The mod owns host authority and main-thread translation; gateway owns
the lifecycle/routing control plane; MCP is a thin adapter; harness owns coordination, experiments, and
artifacts.

## Evidence and provenance

No runtime, loader, game-host, process, transport, provider, or release claim is made here. Static
documentation and governance-tool checks establish preparation only. Future compatibility evidence must
be project-owned, deterministic, exact-versioned, and clearly labeled `confirmed`, `source-derived`,
`proposed`, `inferred`, or `unverified`. Do not copy or retain proprietary game files or another
implementation's source.

Reserved areas are `schemas/domain`, `conformance`, the root `tests` directory, and `tools`.
They are not permission to create empty placeholder crates. The staged scope is recorded in the
workspace planning corpus and is not a product dependency.

## Local validation

The target-local entrypoint is:

```bash
cargo run --locked --package repo-policy -- --strict
```

Run the complete commands in [`docs/TESTING.md`](docs/TESTING.md) after it. The current governance
workspace contains the policy tool and the core package. Passing local checks proves build/test
behavior for this pure seam only; it does not establish host, transport, provider, or runtime claims.
