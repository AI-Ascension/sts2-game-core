// SPDX-License-Identifier: MIT

use crate::combat::{CombatPhase, CombatSnapshot, TurnIndex};
use crate::identity::{Generation, Identity, SessionId};

/// The typed, argument-free action in the frozen Runtime-v2 semantic slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndTurnAction;

impl EndTurnAction {
    /// Creates an end-turn action proposal.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for EndTurnAction {
    fn default() -> Self {
        Self::new()
    }
}

/// A request to evaluate an end-turn action against a combat snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndTurnRequest {
    actor: Identity,
    session: SessionId,
    expected_generation: Generation,
    action: EndTurnAction,
}

impl EndTurnRequest {
    /// Creates a request scoped to an actor, session, and observed generation.
    #[must_use]
    pub const fn new(actor: Identity, session: SessionId, expected_generation: Generation) -> Self {
        Self {
            actor,
            session,
            expected_generation,
            action: EndTurnAction::new(),
        }
    }

    /// Returns the actor that proposed the action.
    #[must_use]
    pub const fn actor(self) -> Identity {
        self.actor
    }

    /// Returns the session that scoped the proposal.
    #[must_use]
    pub const fn session(self) -> SessionId {
        self.session
    }

    /// Returns the generation observed by the proposer.
    #[must_use]
    pub const fn expected_generation(self) -> Generation {
        self.expected_generation
    }

    /// Returns the typed action carried by this request.
    #[must_use]
    pub const fn action(self) -> EndTurnAction {
        self.action
    }
}

/// A request that passed all pure end-turn legality checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedEndTurn {
    actor: Identity,
    session: SessionId,
    generation: Generation,
    action: EndTurnAction,
}

impl ValidatedEndTurn {
    /// Returns the actor associated with the validated proposal.
    #[must_use]
    pub const fn actor(self) -> Identity {
        self.actor
    }

    /// Returns the session associated with the validated proposal.
    #[must_use]
    pub const fn session(self) -> SessionId {
        self.session
    }

    /// Returns the generation against which the proposal was validated.
    #[must_use]
    pub const fn generation(self) -> Generation {
        self.generation
    }

    /// Returns the typed action without performing it.
    #[must_use]
    pub const fn action(self) -> EndTurnAction {
        self.action
    }
}

/// A typed witness that the pure end-turn transition settled in the domain model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndTurnEffectWitness {
    action: EndTurnAction,
    before_generation: Generation,
    after_generation: Generation,
    before_turn: TurnIndex,
    after_turn: TurnIndex,
}

impl EndTurnEffectWitness {
    pub(crate) const fn new(
        action: EndTurnAction,
        before_generation: Generation,
        after_generation: Generation,
        before_turn: TurnIndex,
        after_turn: TurnIndex,
    ) -> Self {
        Self {
            action,
            before_generation,
            after_generation,
            before_turn,
            after_turn,
        }
    }

    /// Returns the action represented by this settled effect.
    #[must_use]
    pub const fn action(self) -> EndTurnAction {
        self.action
    }

    /// Returns the generation before the settled transition.
    #[must_use]
    pub const fn before_generation(self) -> Generation {
        self.before_generation
    }

    /// Returns the generation after the settled transition.
    #[must_use]
    pub const fn after_generation(self) -> Generation {
        self.after_generation
    }

    /// Returns the turn index before the settled transition.
    #[must_use]
    pub const fn before_turn(self) -> TurnIndex {
        self.before_turn
    }

    /// Returns the turn index after the settled transition.
    #[must_use]
    pub const fn after_turn(self) -> TurnIndex {
        self.after_turn
    }
}

/// The immutable result of one settled end-turn domain transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SettledEndTurn {
    snapshot: CombatSnapshot,
    effect: EndTurnEffectWitness,
}

impl SettledEndTurn {
    pub(crate) const fn new(snapshot: CombatSnapshot, effect: EndTurnEffectWitness) -> Self {
        Self { snapshot, effect }
    }

    /// Returns the post-transition snapshot.
    #[must_use]
    pub const fn snapshot(self) -> CombatSnapshot {
        self.snapshot
    }

    /// Returns the typed witness for the settled transition.
    #[must_use]
    pub const fn effect(self) -> EndTurnEffectWitness {
        self.effect
    }
}

/// A deterministic reason an end-turn request was rejected before state change.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndTurnValidationError {
    /// The request actor does not own the snapshot.
    ActorMismatch {
        /// The actor recorded by the snapshot.
        expected: Identity,
        /// The actor carried by the request.
        actual: Identity,
    },
    /// The request session does not scope the snapshot.
    SessionMismatch {
        /// The session recorded by the snapshot.
        expected: SessionId,
        /// The session carried by the request.
        actual: SessionId,
    },
    /// The request targets a different point-in-time generation.
    StaleGeneration {
        /// The generation carried by the request.
        expected: Generation,
        /// The current snapshot generation.
        actual: Generation,
    },
    /// The snapshot is not inside combat.
    OutsideCombat,
    /// The snapshot is in the enemy turn.
    EnemyTurn,
    /// The turn index cannot advance without wrapping.
    TurnIndexExhausted,
    /// The generation cannot advance without wrapping.
    GenerationExhausted,
}

/// A deterministic failure while applying a validated end-turn proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndTurnApplyError {
    /// The proposal actor does not own the snapshot.
    ActorMismatch,
    /// The proposal session does not scope the snapshot.
    SessionMismatch,
    /// The proposal no longer targets the current generation.
    StaleGeneration,
    /// The snapshot is not inside combat.
    OutsideCombat,
    /// The snapshot is in the enemy turn.
    EnemyTurn,
    /// The turn index cannot advance without wrapping.
    TurnIndexExhausted,
    /// The generation cannot advance without wrapping.
    GenerationExhausted,
}

/// Validates an end-turn request against one immutable combat snapshot.
///
/// Checks occur in this order: actor, session, generation, combat phase, turn-index bounds, and
/// generation bounds. A successful result is an accepted semantic proposal, not host execution.
/// Duplicate operation and idempotency decisions remain at the owning boundary; this function has
/// no operation store or side effect.
///
/// # Errors
///
/// Returns an [`EndTurnValidationError`] describing the first failed check.
pub fn validate_end_turn(
    snapshot: &CombatSnapshot,
    request: &EndTurnRequest,
) -> Result<ValidatedEndTurn, EndTurnValidationError> {
    if request.actor != snapshot.actor() {
        return Err(EndTurnValidationError::ActorMismatch {
            expected: snapshot.actor(),
            actual: request.actor,
        });
    }
    if request.session != snapshot.session() {
        return Err(EndTurnValidationError::SessionMismatch {
            expected: snapshot.session(),
            actual: request.session,
        });
    }
    if request.expected_generation != snapshot.generation() {
        return Err(EndTurnValidationError::StaleGeneration {
            expected: request.expected_generation,
            actual: snapshot.generation(),
        });
    }
    match snapshot.phase() {
        CombatPhase::OutsideCombat => return Err(EndTurnValidationError::OutsideCombat),
        CombatPhase::EnemyTurn => return Err(EndTurnValidationError::EnemyTurn),
        CombatPhase::PlayerTurn => {}
    }
    if snapshot.turn_index().next().is_none() {
        return Err(EndTurnValidationError::TurnIndexExhausted);
    }
    if snapshot.generation().next().is_none() {
        return Err(EndTurnValidationError::GenerationExhausted);
    }
    Ok(ValidatedEndTurn {
        actor: request.actor,
        session: request.session,
        generation: request.expected_generation,
        action: request.action,
    })
}
