// SPDX-License-Identifier: MIT

use crate::end_turn::{
    EndTurnAction, EndTurnApplyError, EndTurnEffectWitness, SettledEndTurn, ValidatedEndTurn,
};
use crate::identity::{Generation, Identity, SessionId};

/// The bounded combat phase represented by a semantic snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CombatPhase {
    /// The snapshot does not represent an active combat.
    OutsideCombat,
    /// The player may submit the frozen end-turn action.
    PlayerTurn,
    /// The player may not submit the frozen end-turn action.
    EnemyTurn,
}

/// A bounded ordinal identifying a combat turn.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TurnIndex(u32);

impl TurnIndex {
    /// Creates a turn index from its non-negative numeric representation.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the numeric representation of this turn index.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Advances the index without wrapping at its maximum value.
    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

/// An immutable host-independent combat snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CombatSnapshot {
    actor: Identity,
    session: SessionId,
    generation: Generation,
    phase: CombatPhase,
    turn_index: TurnIndex,
}

impl CombatSnapshot {
    /// Creates a combat snapshot with explicit identity, phase, generation, and turn index.
    #[must_use]
    pub const fn new(
        actor: Identity,
        session: SessionId,
        generation: Generation,
        phase: CombatPhase,
        turn_index: TurnIndex,
    ) -> Self {
        Self {
            actor,
            session,
            generation,
            phase,
            turn_index,
        }
    }

    /// Returns the actor that owns this snapshot.
    #[must_use]
    pub const fn actor(self) -> Identity {
        self.actor
    }

    /// Returns the session that scopes this snapshot.
    #[must_use]
    pub const fn session(self) -> SessionId {
        self.session
    }

    /// Returns the point-in-time generation of this snapshot.
    #[must_use]
    pub const fn generation(self) -> Generation {
        self.generation
    }

    /// Returns the combat phase represented by this snapshot.
    #[must_use]
    pub const fn phase(self) -> CombatPhase {
        self.phase
    }

    /// Returns the combat turn index represented by this snapshot.
    #[must_use]
    pub const fn turn_index(self) -> TurnIndex {
        self.turn_index
    }

    /// Applies a validated end-turn proposal and returns its settled domain witness.
    ///
    /// The operation is pure: the input snapshot is not modified or persisted. The returned
    /// snapshot remains in `combat/player_turn`, with both generation and turn index advanced by
    /// one. A validated proposal is rechecked because an owning boundary may have observed a
    /// newer snapshot before it applies the proposal.
    ///
    /// # Errors
    ///
    /// Returns an [`EndTurnApplyError`] when the proposal no longer matches this snapshot or a
    /// bounded value cannot advance.
    pub fn apply_end_turn(
        &self,
        action: &ValidatedEndTurn,
    ) -> Result<SettledEndTurn, EndTurnApplyError> {
        if action.actor() != self.actor {
            return Err(EndTurnApplyError::ActorMismatch);
        }
        if action.session() != self.session {
            return Err(EndTurnApplyError::SessionMismatch);
        }
        if action.generation() != self.generation {
            return Err(EndTurnApplyError::StaleGeneration);
        }
        match self.phase {
            CombatPhase::OutsideCombat => return Err(EndTurnApplyError::OutsideCombat),
            CombatPhase::EnemyTurn => return Err(EndTurnApplyError::EnemyTurn),
            CombatPhase::PlayerTurn => {}
        }
        let next_generation = self
            .generation
            .next()
            .ok_or(EndTurnApplyError::GenerationExhausted)?;
        let next_turn = self
            .turn_index
            .next()
            .ok_or(EndTurnApplyError::TurnIndexExhausted)?;
        let next = Self {
            generation: next_generation,
            turn_index: next_turn,
            ..*self
        };
        let effect = EndTurnEffectWitness::new(
            EndTurnAction::new(),
            self.generation,
            next_generation,
            self.turn_index,
            next_turn,
        );
        Ok(SettledEndTurn::new(next, effect))
    }
}
