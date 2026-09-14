// SPDX-License-Identifier: MIT

//! Bounded inventory assembly and index helpers.

use super::rules_reference::RuleReference;
use super::rules_reference_ids::RuleId;
use super::rules_reference_records_combat::{
    FIXED_CARD_COST, NOMINAL_CARD_DAMAGE, SYNTHETIC_MODIFIER_ORDERING,
};
use super::rules_reference_records_defense::{
    INCOMING_DAMAGE_AFTER_BLOCK, SATURATING_HEAL_RECOVERY,
};
use super::rules_reference_records_flow::{
    BLOCK_EXPIRY_AT_TURN_START, DRAW_TO_HAND_SIZE, EXHAUST_REMOVES_FROM_ZONE,
    START_OF_TURN_ENERGY_REFILL,
};
use super::rules_reference_records_interactions::{
    CARD_MULTI_HIT_ORDERING, RELIC_FLAT_BONUS_ORDERING,
};
use super::rules_reference_records_scaling::{
    ASCENSION_ENEMY_HP_SCALING, COOP_ENEMY_HP_SCALING, REWARD_CHOICE_PICKS,
};
use super::rules_reference_records_support::{
    POTION_CONSUMPTION_BUDGET, STATUS_VULNERABLE_MULTIPLIER,
};
use super::rules_reference_vocab::RuleFamily;

/// The bounded inventory. Records are grouped by family so one family is always contiguous.
pub(crate) const RULES: &[RuleReference] = &[
    NOMINAL_CARD_DAMAGE,
    SYNTHETIC_MODIFIER_ORDERING,
    FIXED_CARD_COST,
    INCOMING_DAMAGE_AFTER_BLOCK,
    SATURATING_HEAL_RECOVERY,
    DRAW_TO_HAND_SIZE,
    EXHAUST_REMOVES_FROM_ZONE,
    START_OF_TURN_ENERGY_REFILL,
    BLOCK_EXPIRY_AT_TURN_START,
    REWARD_CHOICE_PICKS,
    ASCENSION_ENEMY_HP_SCALING,
    COOP_ENEMY_HP_SCALING,
    CARD_MULTI_HIT_ORDERING,
    RELIC_FLAT_BONUS_ORDERING,
    POTION_CONSUMPTION_BUDGET,
    STATUS_VULNERABLE_MULTIPLIER,
];

/// Finds one record by its stable identifier.
pub(crate) fn find(id: RuleId) -> Option<RuleReference> {
    RULES.iter().copied().find(|rule| rule.id == id)
}

/// Returns the single-record slice owned by one identifier.
pub(crate) fn slice_of(id: RuleId) -> Option<&'static [RuleReference]> {
    let index = RULES.iter().position(|rule| rule.id == id)?;
    RULES.get(index..=index)
}

/// Returns the contiguous records owned by one family, or an empty slice.
pub(crate) fn family_slice(family: RuleFamily) -> &'static [RuleReference] {
    let mut start = None;
    let mut end = 0;
    for (index, rule) in RULES.iter().enumerate() {
        if rule.family == family {
            start.get_or_insert(index);
            end = index;
        }
    }
    let Some(start) = start else {
        return &[];
    };
    let slice = &RULES[start..=end];
    if slice.iter().any(|rule| rule.family != family) {
        return &[];
    }
    slice
}
