// SPDX-License-Identifier: MIT

use super::{
    BuildScope, ContentScope, EntityKind, EntityReference, GameMode, RuleApplicability, RuleId,
    RuleInput, RuleOperation, RuleStep, RuleStepKind, RuleUnit,
};

pub(crate) const ANY_MODES: &[GameMode] = &[GameMode::Any];
pub(crate) const SOLO_MODE: &[GameMode] = &[GameMode::Solo];
pub(crate) const ANY_APPLICABILITY: RuleApplicability = RuleApplicability {
    content: ContentScope::Any,
    build: BuildScope::Any,
    modes: ANY_MODES,
};
pub(crate) const SYNTHETIC_APPLICABILITY: RuleApplicability = RuleApplicability {
    content: ContentScope::Named("synthetic-rules-v1"),
    build: BuildScope::Named("synthetic-build-v1"),
    modes: SOLO_MODE,
};

const fn input(
    name: &'static str,
    unit: RuleUnit,
    required: bool,
    description: &'static str,
) -> RuleInput {
    RuleInput {
        name,
        unit,
        required,
        description,
    }
}
pub(crate) const DAMAGE_INPUTS: &[RuleInput] = &[
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
];
pub(crate) const COST_INPUTS: &[RuleInput] = &[
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
];
pub(crate) const BLOCK_INPUTS: &[RuleInput] = &[
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
];
pub(crate) const SYNTHETIC_DAMAGE_INPUTS: &[RuleInput] = &[
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
];

const fn step(
    order: u8,
    kind: RuleStepKind,
    operation: RuleOperation,
    description: &'static str,
) -> RuleStep {
    RuleStep {
        order,
        kind,
        operation,
        description,
    }
}
pub(crate) const DAMAGE_STEPS: &[RuleStep] = &[step(
    1,
    RuleStepKind::Calculation,
    RuleOperation::Multiply,
    "multiply visible card damage by visible hit count for one target",
)];
pub(crate) const COST_STEPS: &[RuleStep] = &[step(
    1,
    RuleStepKind::Calculation,
    RuleOperation::Subtract,
    "subtract the fixed visible card cost from visible energy",
)];
pub(crate) const BLOCK_STEPS: &[RuleStep] = &[step(
    1,
    RuleStepKind::Calculation,
    RuleOperation::Subtract,
    "subtract player block from caller-resolved incoming damage, saturating at zero",
)];
pub(crate) const SYNTHETIC_DAMAGE_STEPS: &[RuleStep] = &[
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
];

pub(crate) const CARD_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Card,
    id: "card.definition",
};
pub(crate) const SYNTHETIC_CARD_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Card,
    id: "synthetic.card.multi_hit",
};
pub(crate) const SYNTHETIC_RELIC_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Relic,
    id: "synthetic.relic.flat_damage",
};
pub(crate) const SYNTHETIC_STATUS_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Status,
    id: "synthetic.status.multiplier",
};
pub(crate) const PLAYER_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Player,
    id: "player",
};
pub(crate) const CARD_ENTITIES: &[EntityReference<'static>] = &[CARD_ENTITY];
pub(crate) const BLOCK_ENTITIES: &[EntityReference<'static>] = &[PLAYER_ENTITY];
pub(crate) const SYNTHETIC_ENTITIES: &[EntityReference<'static>] = &[
    SYNTHETIC_CARD_ENTITY,
    SYNTHETIC_RELIC_ENTITY,
    SYNTHETIC_STATUS_ENTITY,
];
pub(crate) const NO_RULES: &[RuleId] = &[];
pub(crate) const NOMINAL_RELATED: &[RuleId] = &[];
pub(crate) const SYNTHETIC_RELATED: &[RuleId] = &[RuleId::NominalCardDamage];
