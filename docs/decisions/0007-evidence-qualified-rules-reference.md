# ADR 0007: evidence-qualified rules reference

- Status: Accepted for the host-independent core slice
- Date: 2026-09-13

## Decision

The core package owns inventory version `1` (`RULES_REFERENCE_VERSION`) of a bounded,
host-independent rules reference. A `RuleReference` carries a stable rule ID and mechanic family,
content/build/mode applicability, typed inputs and units, ordered calculation/rounding/targeting/
trigger steps, stack and expiry policy, related entity/rule keys, assumptions, support, and separate
evidence/source labels plus a pinned source reference. The model is declarative and does not execute
a host rule.

The initial records are the existing calculator model: per-target nominal card damage, fixed card
cost, and incoming damage after player block. Each is `SimplifiedModel`, `Synthetic`, and
`Conditional`; all-enemy aggregation, mitigation, powers, statuses, triggers, and game rounding
remain explicitly omitted. A synthetic modifier fixture is included only to test one declared
ordering (flat add, multiplier, floor, then hit aggregation) and is not native/parity evidence.

Family coverage rows inventory damage, block, and resource-cost records as partial. Healing, card
movement, turn/round timing, acquisition, difficulty scaling, co-op scaling, and
card/relic/potion/status interactions are tracked as unmodeled with an exclusion reason.

## Lookup and compatibility

Exact IDs use `rules_for`. `rules_for_mechanic`, `rules_for_entity`, and `lookup_rules` return
bounded matches. A family or entity with omitted combinations returns `Conditional`; an unmodeled
family, unknown entity, compound query, or inapplicable content/build/mode returns `Unsupported`.
No lookup produces a successful partial result labelled complete.

This is additive to the unreleased core API. It has no transport or host dependency and does not
freeze the pending game-information protocol profile. A later accepted protocol contract and
game-mod content-manifest evidence are required before a consumer exposes these records. Tests
establish deterministic metadata and synthetic arithmetic only; native rule parity remains
unverified.
