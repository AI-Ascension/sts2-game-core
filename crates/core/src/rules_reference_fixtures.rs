// SPDX-License-Identifier: MIT

//! Deterministic fixtures for the pure rules-reference inventory.
//!
//! A fixture never claims game or runtime behavior. [`FixtureClass::SimplifiedEstimate`] fixtures
//! mirror the documented simplified calculator model, while
//! [`FixtureClass::ConfirmedSyntheticRule`] fixtures confirm one declared ordered arithmetic against
//! an independently written expected value. Anything outside the declared coverage evaluates to
//! `None` instead of an exact-looking number.

use super::rules_reference_ids::RuleId;
use super::rules_reference_vocab::RoundingRule;

/// Evidence class represented by a deterministic fixture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureClass {
    /// Mirrors the documented simplified calculator model.
    SimplifiedEstimate,
    /// Confirms the declared ordered arithmetic of one synthetic rule record.
    ConfirmedSyntheticRule,
}

/// One ordered arithmetic operation applied by an [`OrderedFixture`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureOp {
    Add(u64),
    SaturatingSubtract(u64),
    Multiply(u64),
    FloorDivide(u64),
    CeilDivide(u64),
    HalfUpDivide(u64),
    ClampMax(u64),
    AtLeast(u64),
    RequireAtMost(u64),
}

/// A deterministic ordered-arithmetic fixture over one declared rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OrderedFixture {
    pub rule: RuleId,
    pub class: FixtureClass,
    pub start: u64,
    pub ops: &'static [FixtureOp],
    pub expected: Option<u64>,
}

impl OrderedFixture {
    /// Applies the declared operations in order, returning `None` outside declared coverage.
    #[must_use]
    pub fn evaluate(self) -> Option<u64> {
        let mut value = self.start;
        for operation in self.ops {
            value = match *operation {
                FixtureOp::Add(amount) => value.checked_add(amount)?,
                FixtureOp::SaturatingSubtract(amount) => value.saturating_sub(amount),
                FixtureOp::Multiply(factor) => value.checked_mul(factor)?,
                FixtureOp::FloorDivide(divisor) => value.checked_div(divisor)?,
                FixtureOp::CeilDivide(divisor) => {
                    let quotient = value.checked_div(divisor)?;
                    quotient + u64::from(!value.is_multiple_of(divisor))
                }
                FixtureOp::HalfUpDivide(divisor) => {
                    let doubled = value.checked_mul(2)?;
                    let doubled_divisor = divisor.checked_mul(2)?;
                    doubled.checked_add(divisor)?.checked_div(doubled_divisor)?
                }
                FixtureOp::ClampMax(limit) => value.min(limit),
                FixtureOp::AtLeast(floor) => value.max(floor),
                FixtureOp::RequireAtMost(limit) => {
                    if value > limit {
                        return None;
                    }
                    value
                }
            };
        }
        Some(value)
    }

    /// Returns whether the fixture reproduces its declared expected value.
    #[must_use]
    pub fn matches_expected(self) -> bool {
        self.evaluate() == self.expected
    }

    /// Returns whether the fixture mirrors the simplified calculator model.
    #[must_use]
    pub const fn is_simplified_estimate(self) -> bool {
        matches!(self.class, FixtureClass::SimplifiedEstimate)
    }
}

/// A fixture for the documented simplified nominal-damage calculator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimplifiedEstimateFixture {
    pub card_damage: u32,
    pub hits: u8,
    pub expected: u32,
}

impl SimplifiedEstimateFixture {
    /// Reproduces the simplified `damage * hits` estimate for one target.
    #[must_use]
    pub fn nominal_damage(self) -> Option<u32> {
        self.card_damage.checked_mul(u32::from(self.hits))
    }
}

/// Synthetic interaction fixture; it carries no host-comparison claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyntheticDamageFixture {
    pub base_damage: u32,
    pub flat_bonus: u32,
    pub multiplier_numerator: u32,
    pub multiplier_denominator: u32,
    pub hits: u8,
    pub rounding: RoundingRule,
    pub expected_damage: u32,
}

impl SyntheticDamageFixture {
    /// Evaluates add, multiply, floor, then hit aggregation with checked arithmetic.
    #[must_use]
    pub fn ordered_damage(self) -> Option<u32> {
        if self.multiplier_denominator == 0
            || self.hits == 0
            || self.rounding != RoundingRule::Floor
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

pub const SIMPLIFIED_NOMINAL_DAMAGE_FIXTURE: SimplifiedEstimateFixture =
    SimplifiedEstimateFixture {
        card_damage: 5,
        hits: 2,
        expected: 10,
    };

pub const SYNTHETIC_MODIFIER_FIXTURE: SyntheticDamageFixture = SyntheticDamageFixture {
    base_damage: 5,
    flat_bonus: 2,
    multiplier_numerator: 3,
    multiplier_denominator: 2,
    hits: 2,
    rounding: RoundingRule::Floor,
    expected_damage: 20,
};

pub const CARD_MULTI_HIT_ORDERING_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::CardMultiHitOrdering,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 5,
    ops: &[
        FixtureOp::Add(2),
        FixtureOp::Multiply(3),
        FixtureOp::FloorDivide(2),
        FixtureOp::Multiply(2),
    ],
    expected: Some(20),
};

pub const RELIC_FLAT_BONUS_ORDERING_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::RelicFlatBonusOrdering,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 5,
    ops: &[
        FixtureOp::Multiply(3),
        FixtureOp::FloorDivide(2),
        FixtureOp::Add(2),
    ],
    expected: Some(9),
};

pub const SATURATING_HEAL_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::SaturatingHealRecovery,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 8,
    ops: &[FixtureOp::Add(5), FixtureOp::ClampMax(9)],
    expected: Some(9),
};

pub const DRAW_TO_HAND_SIZE_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::DrawToHandSize,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 5,
    ops: &[FixtureOp::SaturatingSubtract(2)],
    expected: Some(3),
};

pub const DRAW_WITH_FULL_HAND_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::DrawToHandSize,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 2,
    ops: &[FixtureOp::SaturatingSubtract(5)],
    expected: Some(0),
};

pub const EXHAUST_ZONE_MOVE_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::ExhaustRemovesFromZone,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 3,
    ops: &[FixtureOp::Add(2)],
    expected: Some(5),
};

pub const START_OF_TURN_ENERGY_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::StartOfTurnEnergyRefill,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 2,
    ops: &[FixtureOp::AtLeast(3), FixtureOp::ClampMax(3)],
    expected: Some(3),
};

pub const ENERGY_CAP_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::StartOfTurnEnergyRefill,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 5,
    ops: &[FixtureOp::AtLeast(3), FixtureOp::ClampMax(3)],
    expected: Some(3),
};

pub const POTION_CONSUMPTION_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::PotionConsumptionBudget,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 2,
    ops: &[FixtureOp::SaturatingSubtract(1)],
    expected: Some(1),
};

pub const ASCENSION_ENEMY_HP_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::AscensionEnemyHpScaling,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 40,
    ops: &[FixtureOp::Multiply(12), FixtureOp::CeilDivide(10)],
    expected: Some(48),
};

pub const COOP_ENEMY_HP_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::CoopEnemyHpScaling,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 40,
    ops: &[
        FixtureOp::Multiply(3),
        FixtureOp::Multiply(2),
        FixtureOp::HalfUpDivide(2),
    ],
    expected: Some(120),
};

pub const STATUS_VULNERABLE_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::StatusVulnerableMultiplier,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 7,
    ops: &[FixtureOp::Multiply(3), FixtureOp::FloorDivide(2)],
    expected: Some(10),
};

pub const REWARD_CHOICE_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::RewardChoicePicks,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 1,
    ops: &[FixtureOp::RequireAtMost(3)],
    expected: Some(1),
};

pub const REWARD_CHOICE_OUT_OF_COVERAGE_FIXTURE: OrderedFixture = OrderedFixture {
    rule: RuleId::RewardChoicePicks,
    class: FixtureClass::ConfirmedSyntheticRule,
    start: 4,
    ops: &[FixtureOp::RequireAtMost(3)],
    expected: None,
};
