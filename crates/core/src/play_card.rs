// SPDX-License-Identifier: MIT

use crate::calculators::{CalculatorError, CombatCalculationState, exact_card_damage};
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

/// Observation-derived card facts. Random or hidden effects are not represented.
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

/// A card request that passed pure domain checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedPlayCard {
    request: PlayCardRequest,
    card: CardSpec,
}

impl ValidatedPlayCard {
    #[must_use]
    pub const fn request(self) -> PlayCardRequest {
        self.request
    }

    #[must_use]
    pub const fn card(self) -> CardSpec {
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
    InvalidCard,
    CardNotInHand,
    StaleGeneration,
    InsufficientEnergy,
    InvalidTarget,
    MalformedObservation,
}

impl std::fmt::Display for PlayCardValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidCard => "card facts are invalid",
            Self::CardNotInHand => "card is not in the observed hand",
            Self::StaleGeneration => "card request targets a stale generation",
            Self::InsufficientEnergy => "card cost exceeds observed energy",
            Self::InvalidTarget => "card target is invalid",
            Self::MalformedObservation => "combat observation is malformed",
        })
    }
}

impl std::error::Error for PlayCardValidationError {}

/// Validates one card action against the immutable observation and visible hand.
pub fn validate_play_card(
    state: &CombatCalculationState,
    expected_generation: Generation,
    request: PlayCardRequest,
    hand: &[CardSpec],
) -> Result<ValidatedPlayCard, PlayCardValidationError> {
    state
        .validate()
        .map_err(|_| PlayCardValidationError::MalformedObservation)?;
    if request.generation != expected_generation {
        return Err(PlayCardValidationError::StaleGeneration);
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
    Ok(ValidatedPlayCard { request, card })
}

/// Produces exact visible effect facts without mutating a host state.
pub fn calculate_play_card(
    validated: ValidatedPlayCard,
    state: &CombatCalculationState,
) -> Result<PlayCardFacts, PlayCardValidationError> {
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
