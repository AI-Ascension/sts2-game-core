// SPDX-License-Identifier: MIT

//! Family coverage rows and the explicit inventory of unmodeled combinations.

use super::rules_reference_ids::RuleId;
use super::rules_reference_index as index;
use super::rules_reference_query::{RuleCoverageStatus, RuleFamilyCoverage};
use super::rules_reference_vocab::RuleFamily;

/// One combination that stays outside established coverage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnmodeledCombination {
    pub family: RuleFamily,
    pub rules: &'static [RuleId],
    pub combination: &'static str,
    pub reason: &'static str,
}

const fn coverage(
    family: RuleFamily,
    status: RuleCoverageStatus,
    rules: &'static [super::rules_reference::RuleReference],
    unmodeled: &'static str,
) -> RuleFamilyCoverage {
    RuleFamilyCoverage {
        family,
        status,
        rules,
        unmodeled,
    }
}

const fn exclusion(
    family: RuleFamily,
    rules: &'static [RuleId],
    combination: &'static str,
    reason: &'static str,
) -> UnmodeledCombination {
    UnmodeledCombination {
        family,
        rules,
        combination,
        reason,
    }
}

pub(crate) const FAMILY_COVERAGE: &[RuleFamilyCoverage] = &[
    coverage(
        RuleFamily::Damage,
        RuleCoverageStatus::Partial,
        index::DAMAGE_RULES,
        "mitigation, powers, statuses, triggers, area aggregation, and cross-family combinations",
    ),
    coverage(
        RuleFamily::ResourceCost,
        RuleCoverageStatus::Partial,
        index::RESOURCE_RULES,
        "cost modifiers, alternative payments, and resource triggers",
    ),
    coverage(
        RuleFamily::Block,
        RuleCoverageStatus::Partial,
        index::BLOCK_RULES,
        "enemy block, prevention, thorns, healing, and death replacement",
    ),
    coverage(
        RuleFamily::Heal,
        RuleCoverageStatus::Partial,
        index::HEAL_RULES,
        "healing modifiers, overheal conversion, and revive effects",
    ),
    coverage(
        RuleFamily::CardMovement,
        RuleCoverageStatus::Partial,
        index::CARD_MOVEMENT_RULES,
        "random draw selection, shuffling, and draw/exhaust triggered effects",
    ),
    coverage(
        RuleFamily::TurnTiming,
        RuleCoverageStatus::Partial,
        index::TURN_TIMING_RULES,
        "cross-turn retention, extra-energy sources, and trigger priority",
    ),
    coverage(
        RuleFamily::Acquisition,
        RuleCoverageStatus::Partial,
        index::ACQUISITION_RULES,
        "offered-set generation, rarity weighting, skips, and rerolls",
    ),
    coverage(
        RuleFamily::DifficultyScaling,
        RuleCoverageStatus::Partial,
        index::DIFFICULTY_RULES,
        "encounter, reward, and enemy damage scaling per difficulty step",
    ),
    coverage(
        RuleFamily::CoopScaling,
        RuleCoverageStatus::Partial,
        index::COOP_RULES,
        "peer roster, shared decisions, and per-player reward differences",
    ),
    coverage(
        RuleFamily::CardInteraction,
        RuleCoverageStatus::Partial,
        index::CARD_INTERACTION_RULES,
        "on-hit triggers, mitigation, and cross-family combinations",
    ),
    coverage(
        RuleFamily::RelicInteraction,
        RuleCoverageStatus::Partial,
        index::RELIC_INTERACTION_RULES,
        "relic counters, once-per-turn conditions, and cross-relic ordering",
    ),
    coverage(
        RuleFamily::PotionInteraction,
        RuleCoverageStatus::Partial,
        index::POTION_INTERACTION_RULES,
        "drop weighting, potion-targeted effects, and shared slots",
    ),
    coverage(
        RuleFamily::StatusInteraction,
        RuleCoverageStatus::Partial,
        index::STATUS_INTERACTION_RULES,
        "stack caps, immunity, removal triggers, and status interaction order",
    ),
    coverage(
        RuleFamily::EntityInteraction,
        RuleCoverageStatus::Unmodeled,
        index::ENTITY_INTERACTION_RULES,
        "combinations not explicitly named by a supported record",
    ),
];

pub(crate) const ALL_FAMILIES: &[RuleFamily] = &[
    RuleFamily::Damage,
    RuleFamily::ResourceCost,
    RuleFamily::Block,
    RuleFamily::Heal,
    RuleFamily::CardMovement,
    RuleFamily::TurnTiming,
    RuleFamily::Acquisition,
    RuleFamily::DifficultyScaling,
    RuleFamily::CoopScaling,
    RuleFamily::CardInteraction,
    RuleFamily::RelicInteraction,
    RuleFamily::PotionInteraction,
    RuleFamily::StatusInteraction,
    RuleFamily::EntityInteraction,
];

pub(crate) const UNMODELED_COMBINATIONS: &[UnmodeledCombination] = &[
    exclusion(
        RuleFamily::Damage,
        &[RuleId::NominalCardDamage],
        "nominal card damage resolved against mitigated or modified targets",
        "the simplified calculator record omits mitigation and modifiers",
    ),
    exclusion(
        RuleFamily::Damage,
        &[RuleId::NominalCardDamage, RuleId::SyntheticModifierOrdering],
        "all-enemy aggregation combined with per-target modifiers",
        "the records declare only per-target arithmetic",
    ),
    exclusion(
        RuleFamily::ResourceCost,
        &[RuleId::FixedCardCost],
        "cost modifiers and alternative payment paths",
        "only a fixed visible cost is declared",
    ),
    exclusion(
        RuleFamily::Block,
        &[RuleId::IncomingDamageAfterBlock],
        "block combined with prevention, healing, or death replacement",
        "only player block subtracts from caller-resolved damage",
    ),
    exclusion(
        RuleFamily::Heal,
        &[RuleId::SaturatingHealRecovery],
        "healing combined with modifiers or overheal conversion",
        "only saturating integer recovery is declared",
    ),
    exclusion(
        RuleFamily::CardMovement,
        &[RuleId::DrawToHandSize],
        "draw combined with random selection, shuffling, or draw triggers",
        "core declares no randomness and no trigger ordering",
    ),
    exclusion(
        RuleFamily::CardMovement,
        &[RuleId::ExhaustRemovesFromZone],
        "exhaust combined with on-exhaust triggered effects",
        "only zone bookkeeping is declared",
    ),
    exclusion(
        RuleFamily::TurnTiming,
        &[
            RuleId::StartOfTurnEnergyRefill,
            RuleId::BlockExpiryAtTurnStart,
        ],
        "cross-turn retention effects and trigger priority",
        "boundary records declare one ordered boundary each",
    ),
    exclusion(
        RuleFamily::Acquisition,
        &[RuleId::RewardChoicePicks],
        "reward choice combined with offered-set generation or rerolls",
        "the offered set is unmodeled, so no identity or rarity is inferred",
    ),
    exclusion(
        RuleFamily::DifficultyScaling,
        &[RuleId::AscensionEnemyHpScaling],
        "difficulty scaling combined with encounter or reward changes",
        "only one declared integer hit-point scale is modeled",
    ),
    exclusion(
        RuleFamily::CoopScaling,
        &[RuleId::CoopEnemyHpScaling],
        "co-op scaling combined with shared decisions or peer state",
        "no peer roster, vote, or disagreement resolution is represented",
    ),
    exclusion(
        RuleFamily::CardInteraction,
        &[RuleId::CardMultiHitOrdering],
        "card multi-hit combined with powers, statuses, or on-hit triggers",
        "only the declared ordering and floor rounding are confirmed",
    ),
    exclusion(
        RuleFamily::RelicInteraction,
        &[RuleId::RelicFlatBonusOrdering],
        "relic bonus combined with counters or several relics in one resolution",
        "only one declared post-multiplier bonus is modeled",
    ),
    exclusion(
        RuleFamily::PotionInteraction,
        &[RuleId::PotionConsumptionBudget],
        "potion use combined with drop weighting or shared slots",
        "only the declared consumption budget is modeled",
    ),
    exclusion(
        RuleFamily::StatusInteraction,
        &[RuleId::StatusVulnerableMultiplier],
        "status adjustment combined with stack caps, immunity, or removal triggers",
        "only one declared multiplier with floor rounding is modeled",
    ),
    exclusion(
        RuleFamily::EntityInteraction,
        &[],
        "any combination not named by a supported record",
        "no record claims an unnamed combination, so it stays unsupported",
    ),
];

/// Returns the coverage row for one family.
pub(crate) fn coverage_row(family: RuleFamily) -> Option<RuleFamilyCoverage> {
    FAMILY_COVERAGE
        .iter()
        .find(|row| row.family == family)
        .copied()
}

/// Returns the explicitly unmodeled combinations recorded for one family.
pub(crate) fn exclusions_for_family(family: RuleFamily) -> &'static [UnmodeledCombination] {
    let mut start = None;
    let mut end = 0;
    for (position, entry) in UNMODELED_COMBINATIONS.iter().enumerate() {
        if entry.family == family {
            start.get_or_insert(position);
            end = position;
        }
    }
    let Some(start) = start else {
        return &[];
    };
    let slice = &UNMODELED_COMBINATIONS[start..=end];
    if slice.iter().any(|entry| entry.family != family) {
        return &[];
    }
    slice
}

/// Returns whether any combination for this family stays unmodeled.
pub(crate) fn has_unmodeled_combinations(family: RuleFamily) -> bool {
    !exclusions_for_family(family).is_empty()
}
