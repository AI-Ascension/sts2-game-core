// SPDX-License-Identifier: MIT

//! Stable identifiers for the pure rules-reference records.

use super::rules_reference_vocab::RuleFamily;

/// Stable identifiers for records in the inventory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleId {
    NominalCardDamage,
    SyntheticModifierOrdering,
    FixedCardCost,
    IncomingDamageAfterBlock,
    SaturatingHealRecovery,
    DrawToHandSize,
    ExhaustRemovesFromZone,
    StartOfTurnEnergyRefill,
    BlockExpiryAtTurnStart,
    RewardChoicePicks,
    AscensionEnemyHpScaling,
    CoopEnemyHpScaling,
    CardMultiHitOrdering,
    RelicFlatBonusOrdering,
    PotionConsumptionBudget,
    StatusVulnerableMultiplier,
}

impl RuleId {
    /// Returns the transport-neutral stable key.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NominalCardDamage => "damage.nominal_card",
            Self::SyntheticModifierOrdering => "damage.synthetic_modifier_ordering",
            Self::FixedCardCost => "resource.fixed_card_cost",
            Self::IncomingDamageAfterBlock => "block.incoming_after_block",
            Self::SaturatingHealRecovery => "heal.saturating_recovery",
            Self::DrawToHandSize => "card_movement.draw_to_hand_size",
            Self::ExhaustRemovesFromZone => "card_movement.exhaust_removes_from_zone",
            Self::StartOfTurnEnergyRefill => "turn_timing.start_of_turn_energy_refill",
            Self::BlockExpiryAtTurnStart => "turn_timing.block_expiry_at_turn_start",
            Self::RewardChoicePicks => "acquisition.reward_choice_picks",
            Self::AscensionEnemyHpScaling => "difficulty_scaling.ascension_enemy_hp",
            Self::CoopEnemyHpScaling => "coop_scaling.enemy_hp_by_player_count",
            Self::CardMultiHitOrdering => "card_interaction.multi_hit_ordering",
            Self::RelicFlatBonusOrdering => "relic_interaction.flat_bonus_ordering",
            Self::PotionConsumptionBudget => "potion_interaction.consumption_budget",
            Self::StatusVulnerableMultiplier => "status_interaction.vulnerable_multiplier",
        }
    }

    /// Returns the mechanic family that owns this record.
    #[must_use]
    pub const fn family(self) -> RuleFamily {
        match self {
            Self::NominalCardDamage | Self::SyntheticModifierOrdering => RuleFamily::Damage,
            Self::FixedCardCost => RuleFamily::ResourceCost,
            Self::IncomingDamageAfterBlock => RuleFamily::Block,
            Self::SaturatingHealRecovery => RuleFamily::Heal,
            Self::DrawToHandSize | Self::ExhaustRemovesFromZone => RuleFamily::CardMovement,
            Self::StartOfTurnEnergyRefill | Self::BlockExpiryAtTurnStart => RuleFamily::TurnTiming,
            Self::RewardChoicePicks => RuleFamily::Acquisition,
            Self::AscensionEnemyHpScaling => RuleFamily::DifficultyScaling,
            Self::CoopEnemyHpScaling => RuleFamily::CoopScaling,
            Self::CardMultiHitOrdering => RuleFamily::CardInteraction,
            Self::RelicFlatBonusOrdering => RuleFamily::RelicInteraction,
            Self::PotionConsumptionBudget => RuleFamily::PotionInteraction,
            Self::StatusVulnerableMultiplier => RuleFamily::StatusInteraction,
        }
    }

    /// Returns whether the transport-neutral key identifies this record.
    #[must_use]
    pub fn matches_key(self, key: &str) -> bool {
        self.as_str() == key
    }
}
