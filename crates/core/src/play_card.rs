// SPDX-License-Identifier: MIT

use crate::combat::{CombatPhase, CombatSnapshot};
use crate::identity::{Generation, Identity, SessionId};

/// The maximum hand index accepted by the `runtime-v3-gameplay` profile.
pub const PLAY_CARD_MAX_INDEX: u16 = 64;

/// A host-independent card-play proposal. Card cost and target legality remain host-owned because
/// those facts vary by card model and current combat state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayCardRequest {
    actor: Identity,
    session: SessionId,
    expected_generation: Generation,
    card_index: u16,
    target_id: Option<Identity>,
}

impl PlayCardRequest {
    #[must_use]
    pub const fn new(
        actor: Identity,
        session: SessionId,
        expected_generation: Generation,
        card_index: u16,
        target_id: Option<Identity>,
    ) -> Self {
        Self {
            actor,
            session,
            expected_generation,
            card_index,
            target_id,
        }
    }

    #[must_use]
    pub const fn actor(self) -> Identity {
        self.actor
    }

    #[must_use]
    pub const fn session(self) -> SessionId {
        self.session
    }

    #[must_use]
    pub const fn expected_generation(self) -> Generation {
        self.expected_generation
    }

    #[must_use]
    pub const fn card_index(self) -> u16 {
        self.card_index
    }

    #[must_use]
    pub const fn target_id(self) -> Option<Identity> {
        self.target_id
    }
}

/// A proposal that passed the host-independent portion of card-play legality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedPlayCard {
    request: PlayCardRequest,
}

impl ValidatedPlayCard {
    #[must_use]
    pub const fn request(self) -> PlayCardRequest {
        self.request
    }
}

/// The first deterministic rejection in the pure card-play seam.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayCardValidationError {
    ActorMismatch,
    SessionMismatch,
    StaleGeneration,
    OutsideCombat,
    EnemyTurn,
    CardIndexOutOfRange,
    CardNotInHand,
    GenerationExhausted,
}

/// Checks identity, snapshot freshness, phase, and hand-index bounds without host or transport
/// access. The mod must still revalidate the concrete card model and target immediately before it
/// queues the host action.
///
/// # Errors
///
/// Returns the first deterministic validation error when the request does not match the snapshot,
/// is outside the player-turn phase, or names an unavailable card.
pub fn validate_play_card(
    snapshot: &CombatSnapshot,
    request: &PlayCardRequest,
    hand_count: u16,
) -> Result<ValidatedPlayCard, PlayCardValidationError> {
    if request.actor != snapshot.actor() {
        return Err(PlayCardValidationError::ActorMismatch);
    }
    if request.session != snapshot.session() {
        return Err(PlayCardValidationError::SessionMismatch);
    }
    if request.expected_generation != snapshot.generation() {
        return Err(PlayCardValidationError::StaleGeneration);
    }
    match snapshot.phase() {
        CombatPhase::OutsideCombat => return Err(PlayCardValidationError::OutsideCombat),
        CombatPhase::EnemyTurn => return Err(PlayCardValidationError::EnemyTurn),
        CombatPhase::PlayerTurn => {}
    }
    if request.card_index > PLAY_CARD_MAX_INDEX {
        return Err(PlayCardValidationError::CardIndexOutOfRange);
    }
    if request.card_index >= hand_count {
        return Err(PlayCardValidationError::CardNotInHand);
    }
    if snapshot.generation().next().is_none() {
        return Err(PlayCardValidationError::GenerationExhausted);
    }
    Ok(ValidatedPlayCard { request: *request })
}
