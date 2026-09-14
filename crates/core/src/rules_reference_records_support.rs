// SPDX-License-Identifier: MIT

//! Potion and status interaction records.

use super::rules_reference::{EvidenceStatus, RuleReference, RuleSupport, SourceStatus};
use super::rules_reference_ids::RuleId;
use super::rules_reference_records::{
    ENEMY_ENTITY, NO_RULES, PLAYER_ENTITY, POTION_ENTITY, SYNTHETIC_APPLICABILITY,
    SYNTHETIC_SOURCE, SYNTHETIC_STATUS_ENTITY, input, provenance, reference, semantics, step,
};
use super::rules_reference_vocab::{
    ExpiryRule, RoundingRule, RuleFamily, RuleOperation, RuleStepKind, RuleUnit, StackRule,
    TargetingRule,
};

pub(crate) const POTION_CONSUMPTION_BUDGET: RuleReference = reference(
    RuleId::PotionConsumptionBudget,
    RuleFamily::PotionInteraction,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "slots_held",
            RuleUnit::Count,
            true,
            "declared potions currently held",
        ),
        input(
            "slot_capacity",
            RuleUnit::Count,
            true,
            "declared potion slot capacity",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Targeting,
            RuleOperation::SelectTarget,
            "require the caller to name one held potion",
        ),
        step(
            2,
            RuleStepKind::StateChange,
            RuleOperation::Move,
            "consume the named potion once",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Subtract,
            "decrement the declared held count, saturating at zero",
        ),
    ],
    semantics(
        RoundingRule::None,
        TargetingRule::CallerResolved,
        StackRule::NotApplicable,
        ExpiryRule::Immediate,
        &[POTION_ENTITY, PLAYER_ENTITY],
        NO_RULES,
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "The declared one-potion consumption budget is confirmed by the project-owned potion fixture; drop weighting and shared slots are outside the model. This is not host parity evidence.",
        &[
            "potion drop weighting and reward substitution",
            "potion-targeted and area effects",
            "shared slots across participants",
        ],
    ),
);

pub(crate) const STATUS_VULNERABLE_MULTIPLIER: RuleReference = reference(
    RuleId::StatusVulnerableMultiplier,
    RuleFamily::StatusInteraction,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "incoming_damage",
            RuleUnit::Damage,
            true,
            "declared incoming damage before the status",
        ),
        input(
            "status_numerator",
            RuleUnit::Multiplier,
            true,
            "declared status multiplier numerator",
        ),
        input(
            "status_denominator",
            RuleUnit::Multiplier,
            true,
            "declared status multiplier denominator",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the incoming damage by the declared status numerator",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Divide,
            "divide by the declared status denominator",
        ),
        step(
            3,
            RuleStepKind::Rounding,
            RuleOperation::Round,
            "floor the status-adjusted damage",
        ),
        step(
            4,
            RuleStepKind::StateChange,
            RuleOperation::SetExpiry,
            "expire the status at the declared boundary",
        ),
    ],
    semantics(
        RoundingRule::Floor,
        TargetingRule::SingleEnemy,
        StackRule::Multiplicative,
        ExpiryRule::EndOfTurn,
        &[SYNTHETIC_STATUS_ENTITY, ENEMY_ENTITY],
        &[RuleId::NominalCardDamage],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "The declared multiplicative status adjustment with floor rounding is confirmed by the project-owned status fixture; stack caps and immunity are outside the model. This is not host parity evidence.",
        &[
            "status stack caps and upgrade thresholds",
            "status immunity, resistance, and removal triggers",
            "status multiplication across several statuses",
        ],
    ),
);
