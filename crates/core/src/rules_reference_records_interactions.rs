// SPDX-License-Identifier: MIT

//! Card and relic interaction records.

use super::rules_reference::{EvidenceStatus, RuleReference, RuleSupport, SourceStatus};
use super::rules_reference_ids::RuleId;
use super::rules_reference_records::{
    SYNTHETIC_APPLICABILITY, SYNTHETIC_CARD_ENTITY, SYNTHETIC_RELIC_ENTITY, SYNTHETIC_SOURCE,
    SYNTHETIC_STATUS_ENTITY, input, provenance, reference, semantics, step,
};
use super::rules_reference_vocab::{
    ExpiryRule, RoundingRule, RuleFamily, RuleOperation, RuleStepKind, RuleUnit, StackRule,
    TargetingRule,
};

pub(crate) const CARD_MULTI_HIT_ORDERING: RuleReference = reference(
    RuleId::CardMultiHitOrdering,
    RuleFamily::CardInteraction,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "base_damage",
            RuleUnit::Damage,
            true,
            "declared per-hit base damage",
        ),
        input(
            "flat_bonus",
            RuleUnit::Damage,
            true,
            "declared additive flat bonus",
        ),
        input(
            "multiplier_numerator",
            RuleUnit::Multiplier,
            true,
            "declared multiplier numerator",
        ),
        input(
            "multiplier_denominator",
            RuleUnit::Multiplier,
            true,
            "declared multiplier denominator",
        ),
        input(
            "hit_count",
            RuleUnit::Count,
            true,
            "declared number of hits",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Add,
            "add the flat bonus to the base damage before any multiplier",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the adjusted damage by the declared multiplier numerator",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Divide,
            "divide by the declared multiplier denominator",
        ),
        step(
            4,
            RuleStepKind::Rounding,
            RuleOperation::Round,
            "floor the per-hit damage before hit aggregation",
        ),
        step(
            5,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the rounded per-hit damage by the declared hit count",
        ),
        step(
            6,
            RuleStepKind::Trigger,
            RuleOperation::EmitTrigger,
            "emit the post-damage trigger after the total is settled",
        ),
    ],
    semantics(
        RoundingRule::Floor,
        TargetingRule::SingleEnemy,
        StackRule::Additive,
        ExpiryRule::Immediate,
        &[
            SYNTHETIC_CARD_ENTITY,
            SYNTHETIC_RELIC_ENTITY,
            SYNTHETIC_STATUS_ENTITY,
        ],
        &[RuleId::SyntheticModifierOrdering],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "The declared ordering and floor rounding are confirmed by the project-owned multi-hit fixture; per-hit triggers and mitigation remain outside the model. This is not host parity evidence.",
        &[
            "on-hit and post-damage trigger ordering",
            "target mitigation and enemy block",
            "all-enemy aggregation with per-target modifiers",
        ],
    ),
);

pub(crate) const RELIC_FLAT_BONUS_ORDERING: RuleReference = reference(
    RuleId::RelicFlatBonusOrdering,
    RuleFamily::RelicInteraction,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "base_damage",
            RuleUnit::Damage,
            true,
            "declared per-hit base damage",
        ),
        input(
            "multiplier_numerator",
            RuleUnit::Multiplier,
            true,
            "declared multiplier numerator",
        ),
        input(
            "multiplier_denominator",
            RuleUnit::Multiplier,
            true,
            "declared multiplier denominator",
        ),
        input(
            "flat_bonus",
            RuleUnit::Damage,
            true,
            "declared relic-like flat bonus applied after the multiplier",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the base damage by the declared multiplier numerator",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Divide,
            "divide by the declared multiplier denominator",
        ),
        step(
            3,
            RuleStepKind::Rounding,
            RuleOperation::Round,
            "floor the scaled damage before the flat bonus is applied",
        ),
        step(
            4,
            RuleStepKind::Calculation,
            RuleOperation::Add,
            "add the flat relic bonus to the rounded damage",
        ),
    ],
    semantics(
        RoundingRule::Floor,
        TargetingRule::SingleEnemy,
        StackRule::Additive,
        ExpiryRule::Immediate,
        &[SYNTHETIC_CARD_ENTITY, SYNTHETIC_RELIC_ENTITY],
        &[RuleId::CardMultiHitOrdering],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "The declared post-multiplier bonus ordering is confirmed by the project-owned relic fixture; counter conditions and cross-relic ordering are outside the model. This is not host parity evidence.",
        &[
            "relic counter and once-per-turn conditions",
            "ordering across several relics in one resolution",
            "relic-acquired statuses and their expiry",
        ],
    ),
);
