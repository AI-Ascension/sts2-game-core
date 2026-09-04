# ADR 0007: Bounded Runtime-v3 Card Proposal Validation

- Status: Proposed for the earlier Runtime-v3 gameplay integration
- Date: 2026-09-04

Core owns `PlayCardRequest`, `ValidatedPlayCard`, and deterministic pure validation against an
immutable `CombatSnapshot` and caller-supplied hand count. The intended consumer is the game-mod
boundary. This adds no host, transport, protocol implementation, or persistence dependency.

Checks occur in this order: actor, session, expected generation, combat phase, inclusive index limit
64, index below hand count, and availability of the next domain generation. Rejection returns the
first typed error without modifying input. Successful validation returns the unchanged proposal.
The hand count must be derived from the same snapshot; core cannot independently verify that fact.

An optional target identity is opaque passthrough. Neither its presence nor absence proves that the
card requires a target or that the target is legal. Card existence, cost, target suitability, fresh
host state, main-thread dispatch, and independent settlement evidence remain game-mod responsibilities.
Positional hand indices are point-in-time values, never stable card identities.

This is an additive target-local API at unreleased package version 0.0.0. Existing callers are
unchanged; new consumers must distinguish this bounded admission from complete card legality.
No frozen wire artifact is introduced. PR #6 is an overlapping alternative, not an ancestor or a
drop-in replacement; its API and the selected consumers must be reconciled before combined use.

Tests cover every rejection branch, precedence with several invalid fields, the inclusive index
limit, empty hand, generation exhaustion, target preservation, immutability, and repeated evaluation.
These tests establish this original pure contract, not game-mechanics parity or live execution.
