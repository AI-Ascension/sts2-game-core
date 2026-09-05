// SPDX-License-Identifier: MIT

use crate::calculators::{CalculatorError, CombatCalculationState, exact_card_damage};
use crate::combat::{CombatPhase, CombatSnapshot};
use crate::identity::{Generation, Identity, SessionId};

/// Target domain declared by a visible card definition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetDomain {
    None,
    SelfPlayer,
    SingleEnemy,
    AllEnemies,
}

/// Target selected by a semantic action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardTarget {
    None,
    SelfPlayer,
    Enemy(u32),
    AllEnemies,
}

/// Inputs to the simplified card arithmetic model, not a full game card definition.
/// `card_id` identifies a hand instance. Damage is nominal damage per hit; cost and block are
/// caller-resolved values. Modifiers, target mitigation, random/hidden and triggered effects are
/// not represented. The caller must establish these assumptions before claiming game-exact facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardSpec {
    pub card_id: u32,
    pub cost: u8,
    pub damage: u16,
    pub hits: u8,
    pub block: u16,
    pub target: TargetDomain,
}

impl CardSpec {
    /// Validates a bounded visible card definition.
    ///
    /// # Errors
    ///
    /// Returns an error when the card identity or visible hit definition is invalid.
    pub fn validate(self) -> Result<(), PlayCardValidationError> {
        if self.card_id == 0 || (self.hits == 0 && self.damage > 0) {
            return Err(PlayCardValidationError::InvalidCard);
        }
        Ok(())
    }
}

/// A semantic card-play request bound to one observed generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayCardRequest {
    pub actor: Identity,
    pub session: SessionId,
    pub generation: Generation,
    pub card_id: u32,
    pub target: CardTarget,
}

/// A card request bound to the exact scope, calculation facts, and hand it validated against.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedPlayCard {
    request: PlayCardRequest,
    card: CardSpec,
    snapshot: CombatSnapshot,
    state: CombatCalculationState,
    hand: Vec<CardSpec>,
}

impl ValidatedPlayCard {
    #[must_use]
    pub const fn request(&self) -> PlayCardRequest {
        self.request
    }

    #[must_use]
    pub const fn card(&self) -> CardSpec {
        self.card
    }
}

/// Pure result facts for a valid card play; host settlement remains outside core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayCardFacts {
    pub card_id: u32,
    pub target: CardTarget,
    pub damage: u32,
    pub block: u16,
    pub energy_spent: u8,
}

/// Deterministic validation failures for semantic card play.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayCardValidationError {
    ActorMismatch,
    SessionMismatch,
    OutsideCombat,
    EnemyTurn,
    InvalidCard,
    CardNotInHand,
    StaleGeneration,
    StaleObservation,
    InsufficientEnergy,
    InvalidTarget,
    MalformedObservation,
}

impl std::fmt::Display for PlayCardValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ActorMismatch => "card request actor does not own the snapshot",
            Self::SessionMismatch => "card request session does not scope the snapshot",
            Self::OutsideCombat => "card request is outside combat",
            Self::EnemyTurn => "card request is not in the player turn",
            Self::InvalidCard => "card facts are invalid",
            Self::CardNotInHand => "card is not in the observed hand",
            Self::StaleGeneration => "card request targets a stale generation",
            Self::StaleObservation => "card calculation facts changed after validation",
            Self::InsufficientEnergy => "card cost exceeds observed energy",
            Self::InvalidTarget => "card target is invalid",
            Self::MalformedObservation => "combat observation is malformed",
        })
    }
}

impl std::error::Error for PlayCardValidationError {}

/// Validates one card action against the immutable observation and visible hand.
///
/// # Errors
///
/// Checks actor, session, generation, phase, observation, hand, energy, and target in that order.
/// The snapshot, facts, and hand must come from the same observation; core cannot attest provenance.
/// Returns an error for mismatched scope, phase, malformed or ambiguous hand (maximum 256 cards),
/// unavailable card, insufficient energy, or invalid target. Hand IDs identify instances, not types.
pub fn validate_play_card(
    state: &CombatCalculationState,
    snapshot: &CombatSnapshot,
    request: PlayCardRequest,
    hand: &[CardSpec],
) -> Result<ValidatedPlayCard, PlayCardValidationError> {
    validate_scope(snapshot, request)?;
    state
        .validate()
        .map_err(|_| PlayCardValidationError::MalformedObservation)?;
    if hand.len() > 256 {
        return Err(PlayCardValidationError::MalformedObservation);
    }
    for (index, card) in hand.iter().enumerate() {
        card.validate()?;
        if hand[..index]
            .iter()
            .any(|other| other.card_id == card.card_id)
        {
            return Err(PlayCardValidationError::MalformedObservation);
        }
    }
    let card = hand
        .iter()
        .copied()
        .find(|card| card.card_id == request.card_id)
        .ok_or(PlayCardValidationError::CardNotInHand)?;
    card.validate()?;
    if card.cost > state.energy {
        return Err(PlayCardValidationError::InsufficientEnergy);
    }
    exact_card_damage(&card, state, request.target).map_err(map_calculator_error)?;
    Ok(ValidatedPlayCard {
        request,
        card,
        snapshot: *snapshot,
        state: state.clone(),
        hand: hand.to_vec(),
    })
}

/// Produces exact visible effect facts without mutating a host state.
///
/// # Errors
///
/// Rechecks scope, phase, current hand, energy, and targets. The caller must supply one coherent
/// current observation; this is still only model arithmetic, not permission to mutate a host.
/// Returns an error when the proposal or recorded card no longer matches that observation.
pub fn calculate_play_card(
    validated: &ValidatedPlayCard,
    state: &CombatCalculationState,
    snapshot: &CombatSnapshot,
    hand: &[CardSpec],
) -> Result<PlayCardFacts, PlayCardValidationError> {
    let current = validate_play_card(state, snapshot, validated.request, hand)?;
    if current.card != validated.card {
        return Err(PlayCardValidationError::InvalidCard);
    }
    if current.snapshot != validated.snapshot
        || current.state != validated.state
        || current.hand != validated.hand
    {
        return Err(PlayCardValidationError::StaleObservation);
    }
    let damage = exact_card_damage(&validated.card, state, validated.request.target)
        .map_err(map_calculator_error)?;
    Ok(PlayCardFacts {
        card_id: validated.card.card_id,
        target: validated.request.target,
        damage: damage.damage,
        block: validated.card.block,
        energy_spent: validated.card.cost,
    })
}

fn validate_scope(
    snapshot: &CombatSnapshot,
    request: PlayCardRequest,
) -> Result<(), PlayCardValidationError> {
    if request.actor != snapshot.actor() {
        return Err(PlayCardValidationError::ActorMismatch);
    }
    if request.session != snapshot.session() {
        return Err(PlayCardValidationError::SessionMismatch);
    }
    if request.generation != snapshot.generation() {
        return Err(PlayCardValidationError::StaleGeneration);
    }
    match snapshot.phase() {
        CombatPhase::OutsideCombat => Err(PlayCardValidationError::OutsideCombat),
        CombatPhase::EnemyTurn => Err(PlayCardValidationError::EnemyTurn),
        CombatPhase::PlayerTurn => Ok(()),
    }
}

fn map_calculator_error(error: CalculatorError) -> PlayCardValidationError {
    match error {
        CalculatorError::InvalidTarget => PlayCardValidationError::InvalidTarget,
        CalculatorError::InsufficientEnergy => PlayCardValidationError::InsufficientEnergy,
        CalculatorError::MalformedObservation | CalculatorError::DuplicateTarget => {
            PlayCardValidationError::MalformedObservation
        }
        CalculatorError::InvalidCard | CalculatorError::ArithmeticOverflow => {
            PlayCardValidationError::InvalidCard
        }
    }
}
