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

## Evidence

Focused tests cover exact damage/resource/survival, target-domain and duplicate-target rejection,
stale card generations, and parity between rational belief estimates and weighted enumeration.
They are host-independent evidence. Rules not represented by the checked-in observation model,
including hidden effects and target-build compatibility, remain `unverified`.
