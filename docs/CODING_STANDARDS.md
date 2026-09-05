# Core Coding Standards

## Current implementation stage

Wave 2 initializes the non-empty `sts2-game-core` package under `crates/core`. Its modules contain
only typed semantic values and pure validation. The root Cargo workspace also builds the independent
target-local governance tool. `schemas/domain` and the root `tests` directory remain reserved for
later project-owned contracts; do not fill them with placeholders. The root POC schema and
conformance case are exact protocol-owner mirrors, not new core-owned contracts.

## Rust and module rules

- Use the pinned Rust `1.97.1` toolchain and edition 2024.
- Keep responsibilities in named domain modules; do not create catch-all `utils`, `helpers`,
  `common`, `misc`, `manager`, or `service` modules.
- Prefer private visibility and explicit types. Newtypes and enums are preferred over unscoped
  strings, integers, and boolean combinations for identity, generation, action, and state.
- Validate untrusted values at the semantic boundary and return a structured error.
- Keep core independent of HTTP, MCP, gateway, process, filesystem, clock, provider, and concrete
  host behavior. No network or host callback may appear in a core module.
- Keep unsafe code forbidden. The managed/native and host exception belongs only to `sts2-game-mod`.

## Errors, determinism, and safety

Use `Result` for ordinary failures. Do not use `unwrap`, `expect`, `panic!`, `todo!`, or
`unimplemented!` for input, state, compatibility, or lifecycle conditions. Do not expose debug
strings, paths, or internal type names as contract data. State snapshots and positional indices are
point-in-time values; callers must re-read after a transition or collection-changing action.

Do not use global mutable state, wall-clock reads, random process identity, implicit ordering, or
side effects in semantic validation. If future policy needs time, randomness, or host information,
accept an explicit value or port from the owning boundary and keep that port outside core.

## Budgets and evidence

The policy checker counts nonblank physical lines. Production Rust should stay under 300 lines and
must not exceed 400; Rust test files should stay under 400 and must not exceed 600; workflows should
stay under 160 and must not exceed 200; Markdown should stay under 500 and must not exceed 700.
Refactor by responsibility instead of compressing text. New Rust files need an SPDX MIT header.

Do not copy or transliterate another implementation. Project-owned requirements, original fixtures,
and authorized first-party or black-box evidence define behavior. Label unverified assumptions rather
than encoding them as compatibility facts.

## Review

Every public item needs documentation for behavior, errors, compatibility, and invariants. A change
must explain its owner, consumer, dependency impact, deterministic tests, documentation impact, and
remaining runtime evidence. Run the commands in [`TESTING.md`](TESTING.md) before review.

## Aggregate naming authority

Use the aggregate NAMING_CONVENTIONS.md and naming-registry.yaml for casing,
semantic identity namespaces, lifecycle terms, evidence states, and protected names. Core-owned
domain names remain distinct from protocol, gateway, host, and harness identities even when a suffix
matches.
