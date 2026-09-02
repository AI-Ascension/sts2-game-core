<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/AI-Ascension/.github/main/profile/assets/banner-dark.svg">
  <img alt="AI-Ascension — Inspect how AI requests to a game get fenced, one Rust contract at a time. Runtime: unverified. Deterministic tests: confirmed." src="https://raw.githubusercontent.com/AI-Ascension/.github/main/profile/assets/banner-light.svg" width="100%">
</picture>

# sts2-game-core

> **AI-Ascension · domain core (beside the ascent)** — Host-independent Rust domain core: typed game-state values, pure validation, and policy rules with no I/O.
>
> **Status:** deterministic in-memory tests `confirmed` at the pinned commit · runtime, host, and game compatibility `unverified` · nothing is live.
> **Proof:** [45-second browser replay](https://ai-ascension.github.io/proof.html) · [Evidence ledger](https://ai-ascension.github.io/evidence.html) · [This repository on the map](https://ai-ascension.github.io/repositories.html#sts2-game-core)
> **Owner:** The core maintainers own host-independent STS2 domain state, action descriptions, identity and generation values, validation, typed domain errors, and deterministic policy.
> **Contribute:** [Organization guide](https://github.com/AI-Ascension/.github/blob/main/CONTRIBUTING.md) · [First tasks](https://ai-ascension.github.io/contributing.html)
>
> AI-Ascension is an independent project. It is not affiliated with or endorsed by Mega Crit or Valve and grants no rights to game files, assets, or marks.

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

The package provides typed `Identity`, `SessionId`, `Generation`, `Phase`, `State`, `Action`, and
`Request` values, pure `validate` logic, and checked application of accepted proposals. The POC models
one actor-owned bounded resource: `use_budget` changes available units, generation, and settled-effect
count exactly once; zero units is rejected without a state change. The additive Runtime-v2 seam models
session-scoped `CombatSnapshot` values and the frozen `end_turn` transition from generation 4/turn 2
to generation 5/turn 3 with a typed pure domain witness. Acceptance and pure domain settlement remain
semantic evidence, not host execution.

Before producing a Runtime-v2 observation, use the checked projection: turn index values above 1024
and generations above `9_007_199_254_740_991` are rejected with typed errors rather than silently
clamped or wrapped.

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
They are not permission to create empty placeholder crates. The current target shape is documented in
the [repository layout](docs/REPOSITORY_LAYOUT.md). The staged scope is recorded in the workspace
planning corpus and is not a product dependency.

## Local validation

The target-local entrypoint is:

```bash
cargo run --locked --package repo-policy -- --strict
```

Run the complete commands in [`docs/TESTING.md`](docs/TESTING.md) after it. The current governance
workspace contains the policy tool and the core package. Passing local checks proves build/test
behavior for this pure seam only; it does not establish host, transport, provider, or runtime claims.
