// SPDX-License-Identifier: MIT

//! Vocabulary shared by the pure rules-reference records.

/// Mechanic families, including families tracked as unmodeled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleFamily {
    Damage,
    ResourceCost,
    Block,
    Heal,
    CardMovement,
    TurnTiming,
    Acquisition,
    DifficultyScaling,
    CoopScaling,
    CardInteraction,
    RelicInteraction,
    PotionInteraction,
    StatusInteraction,
    EntityInteraction,
}

/// Terminology alias for [`RuleFamily`].
pub type Mechanic = RuleFamily;

impl RuleFamily {
    /// Returns a stable mechanic key.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Damage => "damage",
            Self::ResourceCost => "resource_cost",
            Self::Block => "block",
            Self::Heal => "heal",
            Self::CardMovement => "card_movement",
            Self::TurnTiming => "turn_timing",
            Self::Acquisition => "acquisition",
            Self::DifficultyScaling => "difficulty_scaling",
            Self::CoopScaling => "coop_scaling",
            Self::CardInteraction => "card_interaction",
            Self::RelicInteraction => "relic_interaction",
            Self::PotionInteraction => "potion_interaction",
            Self::StatusInteraction => "status_interaction",
            Self::EntityInteraction => "entity_interaction",
        }
    }
}

/// Units for rule inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleUnit {
    Damage,
    HitPoints,
    Block,
    Energy,
    Count,
    Turns,
    Rounds,
    Gold,
    Multiplier,
    Ratio,
    Entity,
    Boolean,
    Dimensionless,
}

/// One declared rule input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleInput {
    pub name: &'static str,
    pub unit: RuleUnit,
    pub required: bool,
    pub description: &'static str,
}

/// Kind of an ordered operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleStepKind {
    Calculation,
    Rounding,
    Targeting,
    Trigger,
    StateChange,
}

/// Declarative operation in an ordered step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleOperation {
    Add,
    Multiply,
    Subtract,
    Divide,
    Clamp,
    Round,
    SelectTarget,
    EmitTrigger,
    Move,
    SetExpiry,
}

/// One ordered calculation or trigger step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleStep {
    pub order: u8,
    pub kind: RuleStepKind,
    pub operation: RuleOperation,
    pub description: &'static str,
}

/// Rounding policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundingRule {
    None,
    Floor,
    Ceil,
    HalfUp,
    HalfEven,
    NotRepresented,
}

/// Targeting policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetingRule {
    None,
    SingleEnemy,
    AllLivingEnemies,
    SelfPlayer,
    CallerResolved,
    NotRepresented,
}

/// Repeated-effect stack policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackRule {
    NotApplicable,
    Additive,
    Multiplicative,
    Replace,
    NotRepresented,
}

/// Effect expiry policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpiryRule {
    Immediate,
    EndOfTurn,
    EndOfRound,
    UntilRemoved,
    NotRepresented,
}

/// Entity namespace used by related-reference lookups.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntityKind {
    Card,
    Relic,
    Potion,
    Status,
    Enemy,
    Player,
    Rule,
}

/// A transport-neutral entity key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntityReference<'a> {
    pub kind: EntityKind,
    pub id: &'a str,
}
