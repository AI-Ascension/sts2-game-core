# ADR 0008: rule-family coverage and explicit exclusions

- Status: Accepted for the host-independent core slice
- Date: 2026-09-14

## Context

[ADR 0007](0007-evidence-qualified-rules-reference.md) established inventory version `1`: three
records mirroring the simplified calculators plus one synthetic interaction record. Issue #13 keeps
`sts2-game-core` as the rules-reference feature owner and asks for broader, qualified coverage:
more families, bounded lookup by mechanic and by entity/rule reference, and an explicit inventory of
remaining unmodeled combinations that never yields an exact-looking number.

## Decision

Core owns inventory version `2` (`RULES_REFERENCE_VERSION`) of the same bounded, host-independent
reference. Each record still carries a stable ID, family/mechanic, content/build/mode
applicability, typed inputs and units, ordered calculation/rounding/targeting/trigger steps,
rounding, targeting, stack and expiry policy, related entity/rule keys, evidence and source labels,
plus an explicit `unmodeled` list of combinations the record does not cover.

Covered families are damage, resource cost, block, healing, card movement, turn/round timing,
acquisition, difficulty scaling, co-op scaling, and card/relic/potion/status interactions. Every
record uses integer-only declarative arithmetic; core still executes no host rule, reads no
clock/randomness/provider, and holds no host or transport dependency.

Two evidence classes are deliberately distinguishable:

- `SimplifiedModel`/`Synthetic`/`Conditional` records mirror the documented simplified calculators
  and must never be read as game-exact.
- `Confirmed`/`Synthetic`/`Supported` records confirm one declared ordered arithmetic against a
  project-owned deterministic fixture with an independently written expected value. `Confirmed`
  means the named target-local fixture reproduces the pure declared arithmetic; it is never a host
  comparison. No record uses `SourceStatus::NativeComparison`.

Every fixture is listed in `ORDERED_FIXTURES`, and a coverage test requires every `Confirmed` record
to be reproduced by at least one declared fixture, so a record cannot cite evidence that was never
written.

## Lookup, exclusions, and compatibility

Exact IDs use `rules_for`. `rules_for_mechanic`, `rules_for_entity` (card/relic/potion/status/
enemy/player and `EntityKind::Rule` references), and `lookup_rules` return bounded matches;
`unmodeled_combinations` returns the family-level exclusion inventory and `unmodeled_for` returns one
record's own exclusions. An unmodeled family, unknown entity, unknown rule reference, compound or
context-filtered query, or inapplicable content/build/mode returns `Unsupported`. A family with
remaining exclusions, or a record whose support is conditional, returns `Conditional`. Nothing
converts an unknown request into a zero, an empty description, or a successful partial result
labelled complete.

The inventory is grouped by family so one family is always contiguous, and the entity index is
written out explicitly. Coverage tests recompute both from record metadata, so a record whose family
or `entities` list changes without the index being updated fails validation. Module splits keep each
production file inside the repository size budget.

This is additive to the unreleased core API and does not freeze a game-information protocol profile,
content manifest, or tool shape. Host-parity fixtures, game-mod authoritative-fact adapters, content
provenance, and the authenticated read path remain external gates owned by their repositories.
