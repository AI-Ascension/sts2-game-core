// SPDX-License-Identifier: MIT

//! Explicit entity index for bounded rules-reference lookup.
//!
//! The index is deliberately written out rather than derived so that one entity key maps to one
//! bounded, reviewable record set. `catalog_consistency` tests compare it with the record metadata,
//! so a record whose `entities` list changes without the index being updated fails validation.

use super::rules_reference::RuleReference;
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
use super::rules_reference_vocab::{EntityKind, EntityReference};

/// One indexed entity key and its bounded record set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EntityIndexEntry {
    pub(crate) entity: EntityReference<'static>,
    pub(crate) rules: &'static [RuleReference],
}

const fn entry(
    kind: EntityKind,
    id: &'static str,
    rules: &'static [RuleReference],
) -> EntityIndexEntry {
    EntityIndexEntry {
        entity: EntityReference { kind, id },
        rules,
    }
}

pub(crate) const DAMAGE_RULES: &[RuleReference] =
    &[NOMINAL_CARD_DAMAGE, SYNTHETIC_MODIFIER_ORDERING];
pub(crate) const RESOURCE_RULES: &[RuleReference] = &[FIXED_CARD_COST];
pub(crate) const BLOCK_RULES: &[RuleReference] = &[INCOMING_DAMAGE_AFTER_BLOCK];
pub(crate) const HEAL_RULES: &[RuleReference] = &[SATURATING_HEAL_RECOVERY];
pub(crate) const CARD_MOVEMENT_RULES: &[RuleReference] =
    &[DRAW_TO_HAND_SIZE, EXHAUST_REMOVES_FROM_ZONE];
pub(crate) const TURN_TIMING_RULES: &[RuleReference] =
    &[START_OF_TURN_ENERGY_REFILL, BLOCK_EXPIRY_AT_TURN_START];
pub(crate) const ACQUISITION_RULES: &[RuleReference] = &[REWARD_CHOICE_PICKS];
pub(crate) const DIFFICULTY_RULES: &[RuleReference] = &[ASCENSION_ENEMY_HP_SCALING];
pub(crate) const COOP_RULES: &[RuleReference] = &[COOP_ENEMY_HP_SCALING];
pub(crate) const CARD_INTERACTION_RULES: &[RuleReference] = &[CARD_MULTI_HIT_ORDERING];
pub(crate) const RELIC_INTERACTION_RULES: &[RuleReference] = &[RELIC_FLAT_BONUS_ORDERING];
pub(crate) const POTION_INTERACTION_RULES: &[RuleReference] = &[POTION_CONSUMPTION_BUDGET];
pub(crate) const STATUS_INTERACTION_RULES: &[RuleReference] = &[STATUS_VULNERABLE_MULTIPLIER];
pub(crate) const ENTITY_INTERACTION_RULES: &[RuleReference] = &[];

pub(crate) const CARD_DEFINITION_RULES: &[RuleReference] = &[
    NOMINAL_CARD_DAMAGE,
    FIXED_CARD_COST,
    DRAW_TO_HAND_SIZE,
    EXHAUST_REMOVES_FROM_ZONE,
    REWARD_CHOICE_PICKS,
];
pub(crate) const SYNTHETIC_CARD_RULES: &[RuleReference] = &[
    SYNTHETIC_MODIFIER_ORDERING,
    CARD_MULTI_HIT_ORDERING,
    RELIC_FLAT_BONUS_ORDERING,
];
pub(crate) const SYNTHETIC_RELIC_RULES: &[RuleReference] = &[
    SYNTHETIC_MODIFIER_ORDERING,
    CARD_MULTI_HIT_ORDERING,
    RELIC_FLAT_BONUS_ORDERING,
];
pub(crate) const SYNTHETIC_STATUS_RULES: &[RuleReference] = &[
    SYNTHETIC_MODIFIER_ORDERING,
    CARD_MULTI_HIT_ORDERING,
    STATUS_VULNERABLE_MULTIPLIER,
];
pub(crate) const ENEMY_RULES: &[RuleReference] = &[
    ASCENSION_ENEMY_HP_SCALING,
    COOP_ENEMY_HP_SCALING,
    STATUS_VULNERABLE_MULTIPLIER,
];
pub(crate) const PLAYER_RULES: &[RuleReference] = &[
    INCOMING_DAMAGE_AFTER_BLOCK,
    SATURATING_HEAL_RECOVERY,
    DRAW_TO_HAND_SIZE,
    START_OF_TURN_ENERGY_REFILL,
    BLOCK_EXPIRY_AT_TURN_START,
    REWARD_CHOICE_PICKS,
    COOP_ENEMY_HP_SCALING,
    POTION_CONSUMPTION_BUDGET,
];
pub(crate) const POTION_RULES: &[RuleReference] = &[POTION_CONSUMPTION_BUDGET];

pub(crate) const ENTITY_INDEX: &[EntityIndexEntry] = &[
    entry(EntityKind::Card, "card.definition", CARD_DEFINITION_RULES),
    entry(
        EntityKind::Card,
        "synthetic.card.multi_hit",
        SYNTHETIC_CARD_RULES,
    ),
    entry(
        EntityKind::Relic,
        "synthetic.relic.flat_damage",
        SYNTHETIC_RELIC_RULES,
    ),
    entry(
        EntityKind::Status,
        "synthetic.status.multiplier",
        SYNTHETIC_STATUS_RULES,
    ),
    entry(EntityKind::Enemy, "enemy.definition", ENEMY_RULES),
    entry(EntityKind::Player, "player", PLAYER_RULES),
    entry(EntityKind::Potion, "synthetic.potion", POTION_RULES),
];

/// Returns the bounded record set naming one entity key.
pub(crate) fn find(entity: EntityReference<'_>) -> Option<&'static [RuleReference]> {
    ENTITY_INDEX
        .iter()
        .find(|candidate| candidate.entity.kind == entity.kind && candidate.entity.id == entity.id)
        .map(|candidate| candidate.rules)
}
