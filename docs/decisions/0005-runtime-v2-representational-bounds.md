# ADR 0005: Runtime-v2 Representational Bounds

- Status: Accepted for the target-local representational adapter
- Date: 2026-09-02

## Context

The pure domain model intentionally permits values outside a later representation's numeric
limits. Runtime-v2 observations cannot silently carry values that a safe-integer consumer or the
frozen turn-index contract cannot represent. This constraint must be enforced in `sts2-game-core`
without adding a protocol dependency or changing the existing v1/use_budget behavior.

## Decision

Add a host-independent `runtime_v2` projection module with inclusive limits:

- `RUNTIME_V2_MAX_TURN_INDEX = 1024`;
- `RUNTIME_V2_MAX_GENERATION = 9_007_199_254_740_991` (`2^53 - 1`).

`TurnIndex::try_runtime_v2` and `Generation::try_runtime_v2` return private-field typed wrappers,
`RuntimeV2TurnIndex` and `RuntimeV2Generation`, or a `RuntimeV2ProjectionError`. The wrappers can
also be obtained through their checked `TryFrom` implementations and expose only their checked
numeric values.

`CombatSnapshot::try_runtime_v2` returns a `RuntimeV2Observation` containing the checked wrappers.
It returns an explicit error before producing an observation if either numeric value is outside its
inclusive limit. The projection is read-only and performs no serialization, I/O, persistence, host
access, or retry.

The original `Generation::new`, `TurnIndex::new`, `State`, `Action`, `Request`, `validate`, and
`use_budget` behavior remains unchanged. The domain types may still represent values that are not
valid for Runtime-v2; callers crossing that representational boundary must use the checked
projection.

## Compatibility and evidence

This is an additive-compatible target-local API change. It does not define a wire schema, import
`sts2-protocol`, or add HTTP/host dependencies. Deterministic tests confirm both inclusive maximums,
the first rejected values, equivalent `TryFrom` behavior, observation rejection, read-only
projection, and unchanged domain constructors. Runtime-v2 consumer serialization and host/runtime
compatibility remain unverified.

## Alternatives considered

- **Clamp or wrap values at the projection boundary:** rejected because it would silently change
  domain observations and conceal invalid state.
- **Tighten the existing domain constructors:** rejected because it would change v1 behavior and
  prevent the domain from representing states that later boundaries may reject explicitly.
- **Import a shared protocol crate or serializer:** rejected because this repair owns only the
  host-independent checked representation and must preserve dependency direction.
