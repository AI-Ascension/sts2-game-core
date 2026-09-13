// SPDX-License-Identifier: MIT

//! Public types for the pure, versioned rules-reference model.

/// Version of the core-owned rules-reference inventory.
pub const RULES_REFERENCE_VERSION: u16 = 1;
/// Bound on one collection lookup.
pub const MAX_RULE_MATCHES: usize = 8;

/// Stable identifiers for records in the inventory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleId {
    NominalCardDamage,
    FixedCardCost,
    IncomingDamageAfterBlock,
    SyntheticModifierOrdering,
}

impl RuleId {
    /// Returns the transport-neutral stable key.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NominalCardDamage => "damage.nominal_card",
            Self::FixedCardCost => "resource.fixed_card_cost",
            Self::IncomingDamageAfterBlock => "block.incoming_after_block",
            Self::SyntheticModifierOrdering => "damage.synthetic_modifier_ordering",
        }
    }
}

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

/// Evidence classification. Synthetic values never claim native parity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceStatus {
    SimplifiedModel,
    SyntheticFixture,
    SourceDerived,
    Confirmed,
    Proposed,
    Inferred,
    Unverified,
}
/// Source/provenance classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceStatus {
    Synthetic,
    SourceDerived,
    NativeComparison,
    Unverified,
}
/// Coverage of the declared inputs and operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleSupport {
    Supported,
    Conditional,
    Unsupported,
}

/// Content and build applicability scopes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentScope {
    Any,
    Named(&'static str),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildScope {
    Any,
    Named(&'static str),
}
/// Mode labels accepted by the host-neutral metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameMode {
    Any,
    Solo,
    Cooperative,
}
/// Caller-provided content/build/mode context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleContext<'a> {
    pub content: &'a str,
    pub build: &'a str,
    pub mode: GameMode,
}
/// Applicability attached to a record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleApplicability {
    pub content: ContentScope,
    pub build: BuildScope,
    pub modes: &'static [GameMode],
}

impl RuleApplicability {
    /// Checks all applicability dimensions.
    #[must_use]
    pub fn matches(self, context: RuleContext<'_>) -> bool {
        let content_matches = match self.content {
            ContentScope::Any => true,
            ContentScope::Named(expected) => expected == context.content,
        };
        let build_matches = match self.build {
            BuildScope::Any => true,
            BuildScope::Named(expected) => expected == context.build,
        };
        let mode_matches = self
            .modes
            .iter()
            .any(|candidate| *candidate == GameMode::Any || *candidate == context.mode);
        content_matches && build_matches && mode_matches
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

/// One pure rules-reference record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleReference {
    pub version: u16,
    pub id: RuleId,
    pub family: RuleFamily,
    pub mechanic: Mechanic,
    pub applicability: RuleApplicability,
    pub inputs: &'static [RuleInput],
    pub steps: &'static [RuleStep],
    pub rounding: RoundingRule,
    pub targeting: TargetingRule,
    pub stacking: StackRule,
    pub expiry: ExpiryRule,
    pub entities: &'static [EntityReference<'static>],
    pub related_rules: &'static [RuleId],
    pub evidence: EvidenceStatus,
    pub source: SourceStatus,
    pub source_ref: &'static str,
    pub support: RuleSupport,
    pub assumptions: &'static str,
}

impl RuleReference {
    /// Checks this record's content/build/mode scope.
    #[must_use]
    pub fn applies_to(self, context: RuleContext<'_>) -> bool {
        self.applicability.matches(context)
    }
    /// Checks whether this record names an entity.
    #[must_use]
    pub fn references(self, entity: EntityReference<'_>) -> bool {
        self.entities
            .iter()
            .any(|candidate| candidate.kind == entity.kind && candidate.id == entity.id)
    }
}
