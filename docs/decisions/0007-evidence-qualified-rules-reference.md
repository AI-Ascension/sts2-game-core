# ADR 0007: evidence-qualified rules reference

- Status: Proposed for dependent-contract implementation
- Date: 2026-09-13

## Decision

The core package owns a bounded, pure rules-reference inventory. Each record has a stable rule ID,
mechanic family, ordered calculation steps, assumptions, support state, and evidence status. The
initial records describe only the existing calculator model: nominal card damage, fixed card cost,
and incoming damage after player block. They are `SimplifiedModel` and `Conditional`, never
host-parity rules.

Unsupported families are returned explicitly: healing, card movement, turn timing, acquisition,
difficulty/co-op scaling, and card/relic/potion/status interactions have no evidence-qualified
reference yet. This prevents a caller from turning a missing rule into an exact-looking result.

## Compatibility and evidence

This is additive to the unreleased core API. It has no transport or host dependency and does not
freeze the pending game-information protocol profile. A later accepted protocol contract and
game-mod content-manifest evidence are required before any consumer exposes these records. Tests
confirm the inventory's pure labels and lookup behavior only; native rule parity remains
unverified.
