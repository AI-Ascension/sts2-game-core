// SPDX-License-Identifier: MIT

//! Acquisition, difficulty, and co-op scaling records.

use super::rules_reference::{EvidenceStatus, RuleReference, RuleSupport, SourceStatus};
use super::rules_reference_ids::RuleId;
use super::rules_reference_records::{
    CARD_ENTITY, ENEMY_ENTITY, NO_RULES, PLAYER_ENTITY, SYNTHETIC_APPLICABILITY, SYNTHETIC_SOURCE,
    input, provenance, reference, semantics, step,
};
use super::rules_reference_vocab::{
    ExpiryRule, RoundingRule, RuleFamily, RuleOperation, RuleStepKind, RuleUnit, StackRule,
    TargetingRule,
};

pub(crate) const REWARD_CHOICE_PICKS: RuleReference = reference(
    RuleId::RewardChoicePicks,
    RuleFamily::Acquisition,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "offered_cards",
            RuleUnit::Count,
            true,
            "declared size of the offered set",
        ),
        input(
            "picks",
            RuleUnit::Count,
            true,
            "declared number of chosen cards",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Trigger,
            RuleOperation::EmitTrigger,
            "emit the reward-offered trigger",
        ),
        step(
            2,
            RuleStepKind::Targeting,
            RuleOperation::SelectTarget,
            "select the declared number of picks from the offered set",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Clamp,
            "require the pick count to stay within the offered set size",
        ),
        step(
            4,
            RuleStepKind::StateChange,
            RuleOperation::Move,
            "move the resolved picks into the deck zone",
        ),
    ],
    semantics(
        RoundingRule::None,
        TargetingRule::CallerResolved,
        StackRule::NotApplicable,
        ExpiryRule::Immediate,
        &[CARD_ENTITY, PLAYER_ENTITY],
        NO_RULES,
    ),
    provenance(
        EvidenceStatus::SyntheticFixture,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Conditional,
        "Only the bounded pick count is declared; the offered set, rarity weighting, and card identity are unmodeled and must not be inferred.",
        &[
            "offered-set generation and rarity weighting",
            "skips, rerolls, and gold-based alternatives",
            "card identity and deck placement",
        ],
    ),
);

pub(crate) const ASCENSION_ENEMY_HP_SCALING: RuleReference = reference(
    RuleId::AscensionEnemyHpScaling,
    RuleFamily::DifficultyScaling,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "base_hp",
            RuleUnit::HitPoints,
            true,
            "declared unscaled enemy hit points",
        ),
        input(
            "scaling_steps",
            RuleUnit::Count,
            true,
            "declared difficulty steps applied",
        ),
        input(
            "scaling_numerator",
            RuleUnit::Ratio,
            true,
            "declared numerator of the per-step scale",
        ),
        input(
            "scaling_denominator",
            RuleUnit::Ratio,
            true,
            "declared denominator of the per-step scale",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the per-step numerator by the applied difficulty steps",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Add,
            "add the denominator to form the total scale numerator",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the unscaled hit points by the total scale numerator",
        ),
        step(
            4,
            RuleStepKind::Calculation,
            RuleOperation::Divide,
            "divide the scaled hit points by the declared denominator",
        ),
        step(
            5,
            RuleStepKind::Rounding,
            RuleOperation::Round,
            "round the scaled hit points up",
        ),
    ],
    semantics(
        RoundingRule::Ceil,
        TargetingRule::AllLivingEnemies,
        StackRule::Additive,
        ExpiryRule::Immediate,
        &[ENEMY_ENTITY],
        NO_RULES,
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "Declared integer scaling with ceiling rounding confirmed by the project-owned scaling fixture; it is not a claim about any shipped difficulty setting and not host parity evidence.",
        &[
            "encounter, reward, and card-pool changes per difficulty step",
            "enemy damage and move scaling",
            "non-boss and per-act scaling overrides",
        ],
    ),
);

pub(crate) const COOP_ENEMY_HP_SCALING: RuleReference = reference(
    RuleId::CoopEnemyHpScaling,
    RuleFamily::CoopScaling,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "base_hp",
            RuleUnit::HitPoints,
            true,
            "declared single-player enemy hit points",
        ),
        input(
            "player_count",
            RuleUnit::Count,
            true,
            "declared participating player count",
        ),
        input(
            "hp_share_numerator",
            RuleUnit::Ratio,
            true,
            "declared numerator of the per-player share",
        ),
        input(
            "hp_share_denominator",
            RuleUnit::Ratio,
            true,
            "declared denominator of the per-player share",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the base hit points by the per-player share numerator",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the result by the declared player count",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Divide,
            "divide the adjusted hit points by the declared denominator",
        ),
        step(
            4,
            RuleStepKind::Rounding,
            RuleOperation::Round,
            "round the co-op hit points to the nearest integer, halves upward",
        ),
    ],
    semantics(
        RoundingRule::HalfUp,
        TargetingRule::AllLivingEnemies,
        StackRule::Multiplicative,
        ExpiryRule::Immediate,
        &[ENEMY_ENTITY, PLAYER_ENTITY],
        &[RuleId::AscensionEnemyHpScaling],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "Declared integer co-op scaling with half-up rounding confirmed by the project-owned scaling fixture; peer state and shared decisions are outside the model. This is not host parity evidence.",
        &[
            "peer roster, admission, and reconnection",
            "shared decisions, votes, and disagreement",
            "per-player rewards and ownership differences",
        ],
    ),
);
