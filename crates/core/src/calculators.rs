// SPDX-License-Identifier: MIT

use crate::play_card::{CardSpec, CardTarget, TargetDomain};

/// A player-visible enemy fact used by exact calculators.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnemyFacts {
    pub enemy_id: u32,
    pub hp: u16,
    pub max_hp: u16,
}

/// Host-independent facts copied from one ordinary observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CombatCalculationState {
    pub player_hp: u16,
    pub max_player_hp: u16,
    pub player_block: u16,
    pub incoming_damage: u16,
    pub energy: u8,
    pub enemies: Vec<EnemyFacts>,
}

impl CombatCalculationState {
    /// Validates bounds and duplicate visible enemy identities.
    pub fn validate(&self) -> Result<(), CalculatorError> {
        if self.player_hp > self.max_player_hp || self.enemies.len() > 256 {
            return Err(CalculatorError::MalformedObservation);
        }
        for (index, enemy) in self.enemies.iter().enumerate() {
            if enemy.enemy_id == 0 || enemy.hp > enemy.max_hp {
                return Err(CalculatorError::MalformedObservation);
            }
            if self.enemies[..index]
                .iter()
                .any(|previous| previous.enemy_id == enemy.enemy_id)
            {
                return Err(CalculatorError::DuplicateTarget);
            }
        }
        Ok(())
    }
}

/// Exact damage result derived from visible card and enemy facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExactDamage {
    pub damage: u32,
    pub target_id: Option<u32>,
}

/// Exact resource result for a proposed card play.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExactResourceResult {
    pub energy_before: u8,
    pub energy_after: u8,
    pub energy_spent: u8,
}

/// Exact end-turn survival result from visible incoming damage and block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExactSurvival {
    pub damage_to_player: u16,
    pub hp_after: u16,
    pub survives: bool,
}

/// A deterministic calculator failure; no provider or host is consulted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalculatorError {
    MalformedObservation,
    InvalidCard,
    InvalidTarget,
    DuplicateTarget,
    InsufficientEnergy,
    ArithmeticOverflow,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::MalformedObservation => "observation facts are malformed",
            Self::InvalidCard => "card facts are invalid",
            Self::InvalidTarget => "card target is outside its domain",
            Self::DuplicateTarget => "visible enemy identities are duplicated",
            Self::InsufficientEnergy => "card cost exceeds visible energy",
            Self::ArithmeticOverflow => "exact calculator overflowed its bound",
        })
    }
}

impl std::error::Error for CalculatorError {}

/// Calculates exact damage using only the visible card definition and target facts.
pub fn exact_card_damage(
    card: &CardSpec,
    state: &CombatCalculationState,
    target: CardTarget,
) -> Result<ExactDamage, CalculatorError> {
    state.validate()?;
    card.validate().map_err(|_| CalculatorError::InvalidCard)?;
    validate_target(card.target, state, target)?;
    let damage = u32::from(card.damage)
        .checked_mul(u32::from(card.hits))
        .ok_or(CalculatorError::ArithmeticOverflow)?;
    let target_id = match target {
        CardTarget::Enemy(enemy_id) => Some(enemy_id),
        CardTarget::AllEnemies | CardTarget::SelfPlayer | CardTarget::None => None,
    };
    let total = if card.target == TargetDomain::AllEnemies {
        damage
            .checked_mul(state.enemies.len() as u32)
            .ok_or(CalculatorError::ArithmeticOverflow)?
    } else {
        damage
    };
    Ok(ExactDamage {
        damage: total,
        target_id,
    })
}

/// Calculates exact energy use without mutating the observation.
pub fn exact_resource_after_card(
    card: &CardSpec,
    state: &CombatCalculationState,
) -> Result<ExactResourceResult, CalculatorError> {
    state.validate()?;
    card.validate().map_err(|_| CalculatorError::InvalidCard)?;
    if card.cost > state.energy {
        return Err(CalculatorError::InsufficientEnergy);
    }
    Ok(ExactResourceResult {
        energy_before: state.energy,
        energy_after: state.energy - card.cost,
        energy_spent: card.cost,
    })
}

/// Calculates exact survival after the visible end-turn incoming damage is settled.
pub fn exact_end_turn_survival(
    state: &CombatCalculationState,
) -> Result<ExactSurvival, CalculatorError> {
    state.validate()?;
    let damage_to_player = state.incoming_damage.saturating_sub(state.player_block);
    let hp_after = state.player_hp.saturating_sub(damage_to_player);
    Ok(ExactSurvival {
        damage_to_player,
        hp_after,
        survives: hp_after > 0,
    })
}

/// Checks whether one visible card play is exactly lethal for its selected target.
pub fn exact_lethal(
    card: &CardSpec,
    state: &CombatCalculationState,
    target: CardTarget,
) -> Result<bool, CalculatorError> {
    let damage = exact_card_damage(card, state, target)?.damage;
    match target {
        CardTarget::Enemy(enemy_id) => state
            .enemies
            .iter()
            .find(|enemy| enemy.enemy_id == enemy_id)
            .map(|enemy| damage >= u32::from(enemy.hp))
            .ok_or(CalculatorError::InvalidTarget),
        CardTarget::AllEnemies => {
            let per_enemy = u32::from(card.damage)
                .checked_mul(u32::from(card.hits))
                .ok_or(CalculatorError::ArithmeticOverflow)?;
            Ok(state
                .enemies
                .iter()
                .any(|enemy| per_enemy >= u32::from(enemy.hp)))
        }
        CardTarget::SelfPlayer | CardTarget::None => Ok(false),
    }
}

fn validate_target(
    domain: TargetDomain,
    state: &CombatCalculationState,
    target: CardTarget,
) -> Result<(), CalculatorError> {
    match (domain, target) {
        (TargetDomain::None, CardTarget::None)
        | (TargetDomain::SelfPlayer, CardTarget::SelfPlayer)
        | (TargetDomain::AllEnemies, CardTarget::AllEnemies) => Ok(()),
        (TargetDomain::SingleEnemy, CardTarget::Enemy(enemy_id))
            if state.enemies.iter().any(|enemy| enemy.enemy_id == enemy_id) =>
        {
            Ok(())
        }
        _ => Err(CalculatorError::InvalidTarget),
    }
}
