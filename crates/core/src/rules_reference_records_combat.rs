// SPDX-License-Identifier: MIT

//! Combat records: nominal damage and fixed card cost.

use super::rules_reference::{EvidenceStatus, RuleReference, RuleSupport, SourceStatus};
use super::rules_reference_ids::RuleId;
use super::rules_reference_records::{
    ANY_APPLICABILITY, CARD_ENTITY, NO_RULES, SIMPLIFIED_SOURCE, SYNTHETIC_APPLICABILITY,
    SYNTHETIC_CARD_ENTITY, SYNTHETIC_RELIC_ENTITY, SYNTHETIC_SOURCE, SYNTHETIC_STATUS_ENTITY,
    input, provenance, reference, semantics, step,
};
use super::rules_reference_vocab::{
    ExpiryRule, RoundingRule, RuleFamily, RuleOperation, RuleStepKind, RuleUnit, StackRule,
    TargetingRule,
};

pub(crate) const NOMINAL_CARD_DAMAGE: RuleReference = reference(
    RuleId::NominalCardDamage,
    RuleFamily::Damage,
    ANY_APPLICABILITY,
    &[
        input(
            "card_damage",
            RuleUnit::Damage,
            true,
            "visible nominal damage per hit",
        ),
        input("hit_count", RuleUnit::Count, true, "visible number of hits"),
        input(
            "target",
            RuleUnit::Entity,
            true,
            "one living enemy selected by the caller",
        ),
    ],
    &[step(
        1,
        RuleStepKind::Calculation,
        RuleOperation::Multiply,
        "multiply visible card damage by visible hit count for one target",
    )],
    semantics(
        RoundingRule::NotRepresented,
        TargetingRule::SingleEnemy,
        StackRule::NotRepresented,
        ExpiryRule::Immediate,
        &[CARD_ENTITY],
        NO_RULES,
    ),
    provenance(
        EvidenceStatus::SimplifiedModel,
        SourceStatus::Synthetic,
        SIMPLIFIED_SOURCE,
        RuleSupport::Conditional,
        "This is per-target damage only; all-enemy aggregation, mitigation, powers, statuses, triggers, and rounding are unrepresented.",
        &[
            "all-enemy aggregation",
            "target mitigation",
            "powers and statuses",
            "triggered effects",
            "game rounding",
        ],
    ),
);

pub(crate) const SYNTHETIC_MODIFIER_ORDERING: RuleReference = reference(
    RuleId::SyntheticModifierOrdering,
    RuleFamily::Damage,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "base_damage",
            RuleUnit::Damage,
            true,
            "synthetic visible base damage",
        ),
        input(
            "flat_bonus",
            RuleUnit::Damage,
            true,
            "synthetic relic-like additive modifier",
        ),
        input(
            "multiplier",
            RuleUnit::Multiplier,
            true,
            "synthetic status-like non-negative multiplier",
        ),
        input(
            "hit_count",
            RuleUnit::Count,
            true,
            "synthetic visible hit count",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Add,
            "add the flat modifier to base damage",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the adjusted damage by the status modifier",
        ),
        step(
            3,
            RuleStepKind::Rounding,
            RuleOperation::Round,
            "floor the non-negative adjusted damage before hit aggregation",
        ),
        step(
            4,
            RuleStepKind::Calculation,
            RuleOperation::Multiply,
            "multiply the rounded per-hit damage by visible hit count",
        ),
        step(
            5,
            RuleStepKind::Trigger,
            RuleOperation::EmitTrigger,
            "emit a synthetic post-damage trigger after the total is settled",
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
        &[RuleId::NominalCardDamage],
    ),
    provenance(
        EvidenceStatus::SyntheticFixture,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Conditional,
        "Synthetic interaction only; this record is not native or host-parity evidence.",
        &[
            "mitigation and enemy block",
            "trigger ordering across effects",
            "all-enemy aggregation",
        ],
    ),
);

pub(crate) const FIXED_CARD_COST: RuleReference = reference(
    RuleId::FixedCardCost,
    RuleFamily::ResourceCost,
    ANY_APPLICABILITY,
    &[
        input(
            "energy",
            RuleUnit::Energy,
            true,
            "visible energy before the card",
        ),
        input(
            "card_cost",
            RuleUnit::Energy,
            true,
            "fixed visible card cost",
        ),
    ],
    &[step(
        1,
        RuleStepKind::Calculation,
        RuleOperation::Subtract,
        "subtract the fixed visible card cost from visible energy",
    )],
    semantics(
        RoundingRule::None,
        TargetingRule::None,
        StackRule::NotApplicable,
        ExpiryRule::Immediate,
        &[CARD_ENTITY],
        NO_RULES,
    ),
    provenance(
        EvidenceStatus::SimplifiedModel,
        SourceStatus::Synthetic,
        SIMPLIFIED_SOURCE,
        RuleSupport::Conditional,
        "Cost is supplied as a fixed visible value; cost modifiers, alternative payments, and triggered costs are unrepresented.",
        &[
            "cost modifiers",
            "alternative payments",
            "triggered costs",
            "insufficient-energy rejection ordering",
        ],
    ),
);
