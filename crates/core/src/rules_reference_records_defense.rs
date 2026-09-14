// SPDX-License-Identifier: MIT

//! Defense records: incoming damage after block, and saturating recovery.

use super::rules_reference::{EvidenceStatus, RuleReference, RuleSupport, SourceStatus};
use super::rules_reference_ids::RuleId;
use super::rules_reference_records::{
    ANY_APPLICABILITY, NO_RULES, PLAYER_ENTITY, SIMPLIFIED_SOURCE, SYNTHETIC_APPLICABILITY,
    SYNTHETIC_SOURCE, input, provenance, reference, semantics, step,
};
use super::rules_reference_vocab::{
    ExpiryRule, RoundingRule, RuleFamily, RuleOperation, RuleStepKind, RuleUnit, StackRule,
    TargetingRule,
};

pub(crate) const INCOMING_DAMAGE_AFTER_BLOCK: RuleReference = reference(
    RuleId::IncomingDamageAfterBlock,
    RuleFamily::Block,
    ANY_APPLICABILITY,
    &[
        input(
            "incoming_damage",
            RuleUnit::Damage,
            true,
            "caller-resolved incoming damage",
        ),
        input(
            "player_block",
            RuleUnit::Block,
            true,
            "visible player block",
        ),
    ],
    &[step(
        1,
        RuleStepKind::Calculation,
        RuleOperation::Subtract,
        "subtract player block from caller-resolved incoming damage, saturating at zero",
    )],
    semantics(
        RoundingRule::None,
        TargetingRule::SelfPlayer,
        StackRule::NotApplicable,
        ExpiryRule::Immediate,
        &[PLAYER_ENTITY],
        NO_RULES,
    ),
    provenance(
        EvidenceStatus::SimplifiedModel,
        SourceStatus::Synthetic,
        SIMPLIFIED_SOURCE,
        RuleSupport::Conditional,
        "Incoming damage is caller-resolved; only player block is applied, with no prevention, healing, death replacement, or trigger.",
        &[
            "damage prevention and thorns",
            "enemy block",
            "death replacement",
            "healing and block recovery",
        ],
    ),
);

pub(crate) const SATURATING_HEAL_RECOVERY: RuleReference = reference(
    RuleId::SaturatingHealRecovery,
    RuleFamily::Heal,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "current_hp",
            RuleUnit::HitPoints,
            true,
            "visible current hit points",
        ),
        input(
            "heal_amount",
            RuleUnit::HitPoints,
            true,
            "declared heal amount for this call",
        ),
        input(
            "max_hp",
            RuleUnit::HitPoints,
            true,
            "visible maximum hit points",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Add,
            "add the declared heal amount to the visible current hit points",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Clamp,
            "clamp the recovered total at the visible maximum hit points",
        ),
        step(
            3,
            RuleStepKind::StateChange,
            RuleOperation::Move,
            "apply the bounded recovery to the current hit points",
        ),
    ],
    semantics(
        RoundingRule::None,
        TargetingRule::SelfPlayer,
        StackRule::Additive,
        ExpiryRule::Immediate,
        &[PLAYER_ENTITY],
        &[RuleId::IncomingDamageAfterBlock],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "Declared saturating recovery confirmed by the project-owned heal fixture; integer-only arithmetic. This is not host parity evidence.",
        &[
            "healing modifiers and overheal conversion",
            "enemy and ally healing",
            "death prevention and revive effects",
        ],
    ),
);
