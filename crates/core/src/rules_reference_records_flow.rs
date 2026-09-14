// SPDX-License-Identifier: MIT

//! Zone and timing records: card movement and turn boundaries.

use super::rules_reference::{EvidenceStatus, RuleReference, RuleSupport, SourceStatus};
use super::rules_reference_ids::RuleId;
use super::rules_reference_records::{
    CARD_ENTITY, NO_RULES, PLAYER_ENTITY, SYNTHETIC_APPLICABILITY, SYNTHETIC_SOURCE, input,
    provenance, reference, semantics, step,
};
use super::rules_reference_vocab::{
    ExpiryRule, RoundingRule, RuleFamily, RuleOperation, RuleStepKind, RuleUnit, StackRule,
    TargetingRule,
};

pub(crate) const DRAW_TO_HAND_SIZE: RuleReference = reference(
    RuleId::DrawToHandSize,
    RuleFamily::CardMovement,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "hand_size",
            RuleUnit::Count,
            true,
            "visible cards currently in hand",
        ),
        input(
            "max_hand_size",
            RuleUnit::Count,
            true,
            "declared maximum hand size",
        ),
        input(
            "draw_pile_size",
            RuleUnit::Count,
            true,
            "visible cards remaining in the draw pile",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Calculation,
            RuleOperation::Subtract,
            "subtract the visible hand size from the declared maximum hand size",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Clamp,
            "clamp the difference at zero when the hand is already full",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Clamp,
            "clamp the requested draw to the visible draw pile size",
        ),
        step(
            4,
            RuleStepKind::StateChange,
            RuleOperation::Move,
            "move the resolved number of cards from the draw pile to the hand",
        ),
    ],
    semantics(
        RoundingRule::None,
        TargetingRule::None,
        StackRule::NotApplicable,
        ExpiryRule::Immediate,
        &[CARD_ENTITY, PLAYER_ENTITY],
        NO_RULES,
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "Declared bounded draw arithmetic confirmed by the project-owned draw fixtures; draw randomness and triggers are outside the model. This is not host parity evidence.",
        &[
            "random draw selection and shuffling",
            "empty-pile and fatigue effects",
            "draw-triggered card effects",
        ],
    ),
);

pub(crate) const EXHAUST_REMOVES_FROM_ZONE: RuleReference = reference(
    RuleId::ExhaustRemovesFromZone,
    RuleFamily::CardMovement,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "exhausted_cards",
            RuleUnit::Count,
            true,
            "declared number of exhausted cards",
        ),
        input(
            "discard_pile_size",
            RuleUnit::Count,
            true,
            "visible cards in the discard pile",
        ),
        input(
            "exhaust_pile_size",
            RuleUnit::Count,
            true,
            "visible cards already exhausted this combat",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::StateChange,
            RuleOperation::Move,
            "remove the declared cards from the discard and draw zones",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Subtract,
            "subtract the removed cards from the discard pile size",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Add,
            "add the removed cards to the exhaust pile size",
        ),
    ],
    semantics(
        RoundingRule::None,
        TargetingRule::None,
        StackRule::NotApplicable,
        ExpiryRule::UntilRemoved,
        &[CARD_ENTITY],
        &[RuleId::DrawToHandSize],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "Declared zone bookkeeping confirmed by the project-owned exhaust fixture; exhaust triggers and cross-zone ordering are outside the model. This is not host parity evidence.",
        &[
            "on-exhaust triggered effects",
            "zone-order and shuffle bookkeeping",
            "cards that leave combat zones by other means",
        ],
    ),
);

pub(crate) const START_OF_TURN_ENERGY_REFILL: RuleReference = reference(
    RuleId::StartOfTurnEnergyRefill,
    RuleFamily::TurnTiming,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "energy_before",
            RuleUnit::Energy,
            true,
            "visible energy carried into the turn boundary",
        ),
        input(
            "base_energy",
            RuleUnit::Energy,
            true,
            "declared base energy for one turn",
        ),
        input(
            "energy_cap",
            RuleUnit::Energy,
            true,
            "declared maximum energy",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Trigger,
            RuleOperation::EmitTrigger,
            "emit the start-of-turn boundary trigger",
        ),
        step(
            2,
            RuleStepKind::Calculation,
            RuleOperation::Clamp,
            "keep the higher of the carried energy and the declared base energy",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Clamp,
            "clamp the refilled energy at the declared maximum",
        ),
    ],
    semantics(
        RoundingRule::None,
        TargetingRule::SelfPlayer,
        StackRule::Replace,
        ExpiryRule::EndOfTurn,
        &[PLAYER_ENTITY],
        &[RuleId::FixedCardCost],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "Declared energy refill confirmed by the project-owned energy fixtures; energy modifiers and carry rules beyond the declared bound are outside the model. This is not host parity evidence.",
        &[
            "extra-energy relics and modifiers",
            "energy debt and cross-turn retention effects",
            "start-of-turn trigger ordering across effects",
        ],
    ),
);

pub(crate) const BLOCK_EXPIRY_AT_TURN_START: RuleReference = reference(
    RuleId::BlockExpiryAtTurnStart,
    RuleFamily::TurnTiming,
    SYNTHETIC_APPLICABILITY,
    &[
        input(
            "block_before",
            RuleUnit::Block,
            true,
            "visible block before the turn boundary",
        ),
        input(
            "expiring_block",
            RuleUnit::Block,
            true,
            "declared block granted before the boundary",
        ),
    ],
    &[
        step(
            1,
            RuleStepKind::Trigger,
            RuleOperation::EmitTrigger,
            "emit the start-of-turn boundary trigger",
        ),
        step(
            2,
            RuleStepKind::StateChange,
            RuleOperation::SetExpiry,
            "expire the block granted before the boundary",
        ),
        step(
            3,
            RuleStepKind::Calculation,
            RuleOperation::Clamp,
            "clamp the remaining block at zero",
        ),
    ],
    semantics(
        RoundingRule::None,
        TargetingRule::SelfPlayer,
        StackRule::Replace,
        ExpiryRule::EndOfTurn,
        &[PLAYER_ENTITY],
        &[RuleId::IncomingDamageAfterBlock],
    ),
    provenance(
        EvidenceStatus::Confirmed,
        SourceStatus::Synthetic,
        SYNTHETIC_SOURCE,
        RuleSupport::Supported,
        "Declared block expiry confirmed by the project-owned expiry fixture; retention effects and block conversion are outside the model. This is not host parity evidence.",
        &[
            "retention effects that keep block across turns",
            "block conversion and retaliation effects",
            "enemy block expiry timing",
        ],
    ),
);
