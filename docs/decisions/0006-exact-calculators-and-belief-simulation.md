# ADR 0006: observation-derived calculators and explicit belief simulation

- Status: Accepted for the host-independent rules package; target-build compatibility remains unverified
- Date: 2026-09-04

## Decision

`sts2-game-core` owns exact calculations that can be derived from an ordinary fair-play observation:
target-domain validation, visible card damage, resource expenditure, incoming damage after block,
lethal witnesses, and survival. These functions are pure and do not access a host, transport,
process, clock, provider, seed, or future RNG.

Unknown randomness is represented only by caller-supplied `BeliefState` branches with positive
weights. The simulator enumerates those explicit branches and labels results `ExplicitBelief`; it
does not turn a visible seed into a prediction or claim that an estimate is an exact game fact.
The game-mod remains authoritative for actual legality, mutation, and settlement.

## Safety correction and limits

The unreleased `validate_play_card` API now takes the existing `CombatSnapshot`, not a bare
generation, and checks actor, session, generation and player-turn phase before fact/hand validation.
Hands have at most 256 unique instance IDs. `ValidatedPlayCard` owns the original snapshot, facts
and hand; `calculate_play_card` borrows it and rechecks the supplied current values. Changed facts
under the same generation are rejected, not silently substituted. Callers must pass values from
one coherent observation and cannot use the result to bypass host admission. This is an unreleased
Rust API safety correction; no wire artifact is changed and no production consumer of these APIs
has been identified in the reviewed Runtime-v3 mod proposal.

These are exact arithmetic operations **within a simplified model**, not verified STS2 rule parity:
nominal damage is `damage * hits`, summed over supplied enemies for area damage; target mitigation,
statuses, triggers and other card effects are absent. Incoming damage is caller-resolved except for
player block, cost is fixed, and no healing or death prevention occurs. All-enemy lethal means at
least one supplied target reaches its HP threshold, not an encounter victory. Zero-HP entries are
accepted as facts; callers decide whether such entries belong in their target set. When omitted
effects matter, outputs must not be advertised as exact game predictions. Belief branches also
inherit these assumptions; supplied weights have no empirical calibration claim.

## Evidence

Focused tests cover exact damage/resource/survival, target-domain and duplicate-target rejection,
stale card generations, and parity between rational belief estimates and weighted enumeration.
They are host-independent evidence. Rules not represented by the checked-in observation model,
including hidden effects and target-build compatibility, remain `unverified`.
