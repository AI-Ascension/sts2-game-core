# ADR 0004: Runtime-v2 Pure End-turn Semantic Action

- Status: Accepted for the target-local pure semantic slice
- Date: 2026-09-02

## Context

Runtime-v1 proves a small `use_budget` state transition, but it does not model a combat phase,
session scope, or an end-turn domain witness. Runtime-v2 needs one frozen semantic action that can
be implemented and tested before any mod, transport, gateway, MCP, or harness work. The action must
remain host-independent and must not imply that a core result is a settled host mutation.

## Decision

Add an additive combat domain seam under `crates/core` with these project-owned types:

- `SessionId` is a typed, non-zero session identity alongside the existing `Identity` and
  `Generation` values.
- `CombatSnapshot` is an immutable snapshot containing actor, session, generation, combat phase,
  and turn index.
- `EndTurnAction` is a typed, argument-free action. `EndTurnRequest` carries the actor, session,
  and expected generation used for validation.
- `validate_end_turn` returns a `ValidatedEndTurn` proposal or a typed rejection.
- `CombatSnapshot::apply_end_turn` returns `SettledEndTurn`, which contains the next snapshot and
  an `EndTurnEffectWitness` describing the before/after generation and turn index.

An end-turn request is legal only when the snapshot is `CombatPhase::PlayerTurn`. Validation checks
actor identity, session identity, generation freshness, combat phase, turn-index bounds, and
generation bounds in that order. Rejections leave the supplied snapshot unchanged. In particular,
outside-combat, enemy-turn, stale-generation, identity-mismatch, and exhausted-bound cases are
represented before a transition is returned.

The deterministic fake transition is:

```text
combat/player_turn, generation 4, turn index 2
    -> combat/player_turn, generation 5, turn index 3
```

The returned witness is domain settlement evidence only. The operation is immutable and has no
filesystem, persistence, clock, host, network, or provider side effect. Applying the same
validated proposal to the returned generation is rejected as stale rather than applying a second
transition.

Operation identity, duplicate detection, receipt retention, timeout uncertainty, and
idempotency-conflict decisions belong to the owning boundary. Core exposes no operation store and
does not retry or replay requests. A later gateway/MCP/harness contract must represent those
decisions explicitly and must not infer them from this pure semantic result.

## Compatibility and evidence

This is an additive-compatible change for existing `State`, `Action`, `Request`, and `validate`
behavior. The existing `use_budget` seam and copied POC artifact verification remain unchanged.
The new API is a target-local semantic contract; it is not a wire shape and does not add a
protocol dependency.

Deterministic tests confirm the valid transition, typed witness, identity/session/generation and
phase/bounds rejection precedence, repeated evaluation stability, input immutability, and one-time
application against a newer generation. Host action feasibility, managed main-thread execution,
transport mapping, operation reconciliation, and durable persistence behavior remain unverified and
are owned by later boundaries.

## Alternatives considered

- **Add `EndTurn` to the existing public `Action` enum:** rejected because it would force existing
  exhaustive consumers to change for a separate combat seam.
- **Reuse profile or save mutation as the action:** rejected because persistence is outside core and
  does not represent an in-run combat transition.
- **Add a protocol, operation ledger, or transport adapter here:** rejected because those concerns
  belong to the accepted neutral-contract and control-plane owners, not the pure semantic core.
