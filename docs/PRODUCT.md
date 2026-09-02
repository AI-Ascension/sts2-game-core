# Core Product Contract

## Purpose and owner

`sts2-game-core` is the semantic foundation of the STS2 build: an original Rust boundary for
host-independent domain state, action proposals, identity, generation/freshness, validation, typed
errors, and deterministic policy. The core maintainers own these concepts. The future game mod is the
primary consumer; owner-local tests and explicitly accepted neutral protocol contracts are secondary
consumers.

This repository is greenfield. No source compatibility, implementation lineage, or behavior is
inherited from another harness. A public contract exists only after project-owned requirements,
reviewed shapes/fixtures where appropriate, and deterministic acceptance tests exist.

## Initialized scope

Wave 2 adds the non-empty `sts2-game-core` package at `crates/core`. Its initial pure semantic seam
defines explicit types for:

- point-in-time state snapshots and their generation/freshness scope;
- actor or identity values whose lifetime and authority are explicit;
- action names and typed arguments without host object references;
- validation outcomes that distinguish malformed, stale, illegal, and accepted proposals; and
- deterministic validation precedence and structured domain errors.

The seam models an actor-owned bounded resource and open/closed lifecycle as a deliberately small
semantic test contract. It does not claim to reproduce game mechanics and is not a frozen wire shape.
Validation is read-only: acceptance returns a proposal for the owning boundary, not execution.

## Non-goals

Core does not expose HTTP or MCP, own gateway lifecycle/routing, start or supervise processes, read
or write files/saves, obtain wall-clock time, call providers, load the game, bind host objects, own
loader/FFI or main-thread dispatch, or claim an accepted host mutation completed. The mod retains
host authority; gateway is the lifecycle/routing control plane; MCP is a thin adapter; harness is the
coordinator and experiment/artifact owner.

## Contract and evidence rules

Before adding a public value or rule, document success, rejection, stale-input behavior, error shape,
ordering, compatibility class, and test oracle. Keep protocol scope limited to genuinely shared
language- and transport-neutral contracts. Label runtime, host, provider, and packaging evidence
`unverified` until reproduced in the owning boundary.
