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

The frozen Runtime-v2 semantic slice adds a session-scoped `CombatSnapshot`, an argument-free
`end_turn` action, phase/generation/bounds validation, and an immutable settled domain witness for
the deterministic `generation 4 / turn 2 -> generation 5 / turn 3` transition. This is pure core
settlement, not evidence of a host action.

Its representational adapter requires callers to check `TurnIndex` against an inclusive maximum of
1024 and `Generation` against the safe-integer maximum of `9_007_199_254_740_991` before producing a
Runtime-v2 observation. Out-of-range values return typed errors; they are never clamped or wrapped.

The seam models an actor-owned bounded resource and open/closed lifecycle as a deliberately small
semantic test contract. It does not claim to reproduce game mechanics and is not a frozen wire shape.
Validation is read-only: acceptance returns a proposal for the owning boundary, not execution.

## Non-goals

The proposed earlier Runtime-v3 `play_card` seam validates only actor, session, freshness, combat
phase, hand-index bounds, and generation availability. It carries an optional target identity
without validating target legality and returns an unchanged proposal, not execution or settlement.
See [ADR 0007](decisions/0007-runtime-v3-card-action-semantics.md).

Core does not expose HTTP or MCP, own gateway lifecycle/routing, start or supervise processes, read
or write files/saves, obtain wall-clock time, call providers, load the game, bind host objects, own
loader/FFI or main-thread dispatch, or claim an accepted host mutation completed. The mod retains
host authority; gateway is the lifecycle/routing control plane; MCP is a thin adapter; harness is the
coordinator and experiment/artifact owner.

Core also does not own operation receipts, duplicate/idempotency storage, timeout reconciliation, or
retry policy. Those boundary decisions must be explicit when later consumers carry this action.

## Contract and evidence rules

Before adding a public value or rule, document success, rejection, stale-input behavior, error shape,
ordering, compatibility class, and test oracle. Keep protocol scope limited to genuinely shared
language- and transport-neutral contracts. Label runtime, host, provider, and packaging evidence
`unverified` until reproduced in the owning boundary.
