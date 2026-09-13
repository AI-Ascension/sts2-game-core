// SPDX-License-Identifier: MIT

use super::rules_reference::{EntityReference, RuleContext, RuleFamily, RuleId, RuleReference};
use super::rules_reference_catalog as catalog;

/// Result of an exact rule-ID lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleLookup {
    Found(RuleReference),
    Unsupported {
        family: RuleFamily,
        reason: &'static str,
    },
}

/// A bounded set returned by a family/entity query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleMatches {
    pub rules: &'static [RuleReference],
}

impl RuleMatches {
    /// Returns records in this bounded result.
    #[must_use]
    pub const fn as_slice(self) -> &'static [RuleReference] {
        self.rules
    }
    /// Returns the number of records in this result.
    #[must_use]
    pub const fn len(self) -> usize {
        self.rules.len()
    }
    /// Returns whether no records matched.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.rules.is_empty()
    }
}

/// Collection lookup status; conditional results are never complete game claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleCollectionLookup {
    Found(RuleMatches),
    Conditional {
        matches: RuleMatches,
        reason: &'static str,
    },
    Unsupported {
        family: RuleFamily,
        reason: &'static str,
    },
}

impl RuleCollectionLookup {
    /// Returns whether the result is conditional rather than unsupported.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(self, Self::Conditional { .. })
    }
}

/// Query by one indexed dimension.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RuleQuery<'a> {
    pub rule_id: Option<RuleId>,
    pub mechanic: Option<RuleFamily>,
    pub entity: Option<EntityReference<'a>>,
    pub context: Option<RuleContext<'a>>,
}

impl<'a> RuleQuery<'a> {
    /// Creates an exact rule query.
    #[must_use]
    pub const fn by_id(rule_id: RuleId) -> Self {
        Self {
            rule_id: Some(rule_id),
            mechanic: None,
            entity: None,
            context: None,
        }
    }
    /// Creates a mechanic query.
    #[must_use]
    pub const fn by_mechanic(mechanic: RuleFamily) -> Self {
        Self {
            rule_id: None,
            mechanic: Some(mechanic),
            entity: None,
            context: None,
        }
    }
    /// Creates an entity query.
    #[must_use]
    pub const fn by_entity(entity: EntityReference<'a>) -> Self {
        Self {
            rule_id: None,
            mechanic: None,
            entity: Some(entity),
            context: None,
        }
    }
    /// Adds an applicability context.
    #[must_use]
    pub const fn in_context(mut self, context: RuleContext<'a>) -> Self {
        self.context = Some(context);
        self
    }
}

/// Family inventory status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleCoverageStatus {
    Partial,
    Unmodeled,
}

/// Explicit record of modeled and remaining family coverage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleFamilyCoverage {
    pub family: RuleFamily,
    pub status: RuleCoverageStatus,
    pub rules: &'static [RuleId],
    pub unmodeled: &'static str,
}

/// Synthetic interaction fixture; it carries no host-parity claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyntheticDamageFixture {
    pub base_damage: u32,
    pub flat_bonus: u32,
    pub multiplier_numerator: u32,
    pub multiplier_denominator: u32,
    pub hits: u8,
    pub rounding: super::rules_reference::RoundingRule,
    pub expected_damage: u32,
}

impl SyntheticDamageFixture {
    /// Evaluates add, multiply, floor, then hit aggregation with checked arithmetic.
    #[must_use]
    pub fn ordered_damage(self) -> Option<u32> {
        if self.multiplier_denominator == 0
            || self.hits == 0
            || self.rounding != super::rules_reference::RoundingRule::Floor
        {
            return None;
        }
        let adjusted = self.base_damage.checked_add(self.flat_bonus)?;
        let scaled = adjusted.checked_mul(self.multiplier_numerator)?;
        scaled
            .checked_div(self.multiplier_denominator)?
            .checked_mul(u32::from(self.hits))
    }
}

const fn coverage(
    family: RuleFamily,
    status: RuleCoverageStatus,
    rules: &'static [RuleId],
    unmodeled: &'static str,
) -> RuleFamilyCoverage {
    RuleFamilyCoverage {
        family,
        status,
        rules,
        unmodeled,
    }
}

pub(crate) const FAMILY_COVERAGE: &[RuleFamilyCoverage] = &[
    coverage(
        RuleFamily::Damage,
        RuleCoverageStatus::Partial,
        catalog::DAMAGE_IDS,
        "mitigation, powers, statuses, triggers, area aggregation, and cross-family combinations",
    ),
    coverage(
        RuleFamily::ResourceCost,
        RuleCoverageStatus::Partial,
        catalog::COST_IDS,
        "cost modifiers, alternative payments, and resource triggers",
    ),
    coverage(
        RuleFamily::Block,
        RuleCoverageStatus::Partial,
        catalog::BLOCK_IDS,
        "enemy block, prevention, thorns, healing, and death replacement",
    ),
    coverage(
        RuleFamily::Heal,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "all healing and recovery interactions",
    ),
    coverage(
        RuleFamily::CardMovement,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "draw, discard, exhaust, retain, shuffle, and zone timing",
    ),
    coverage(
        RuleFamily::TurnTiming,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "turn/round boundaries, start/end triggers, and priority",
    ),
    coverage(
        RuleFamily::Acquisition,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "rewards, shops, unlocks, and generated acquisition choices",
    ),
    coverage(
        RuleFamily::DifficultyScaling,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "ascension and encounter difficulty scaling",
    ),
    coverage(
        RuleFamily::CoopScaling,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "co-op player count, shared effects, and peer state",
    ),
    coverage(
        RuleFamily::CardInteraction,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "card-specific alternate costs, effects, and movement",
    ),
    coverage(
        RuleFamily::RelicInteraction,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "relic triggers and relic-modified card or combat effects",
    ),
    coverage(
        RuleFamily::PotionInteraction,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "potion targeting, consumption, and potion-triggered effects",
    ),
    coverage(
        RuleFamily::StatusInteraction,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
        "status stacking, expiry, and status-triggered effects",
    ),
    coverage(
        RuleFamily::EntityInteraction,
        RuleCoverageStatus::Unmodeled,
        catalog::NO_IDS,
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

/// A deterministic synthetic fixture for modifier ordering and rounding tests.
pub const SYNTHETIC_MODIFIER_FIXTURE: SyntheticDamageFixture = SyntheticDamageFixture {
    base_damage: 5,
    flat_bonus: 2,
    multiplier_numerator: 3,
    multiplier_denominator: 2,
    hits: 2,
    rounding: super::rules_reference::RoundingRule::Floor,
    expected_damage: 20,
};
