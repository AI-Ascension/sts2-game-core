// SPDX-License-Identifier: MIT

/// A stable, host-independent identifier for one documented rule reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleId {
    NominalCardDamage,
    FixedCardCost,
    IncomingDamageAfterBlock,
}

/// Broad mechanic families deliberately distinguished from individual rule references.
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
    EntityInteraction,
}

/// The evidence boundary for a rule. `SimplifiedModel` is never host-parity evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceStatus {
    SimplifiedModel,
    Unverified,
}

/// Whether a requested combination is represented by the bounded reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleSupport {
    Supported,
    Conditional,
    Unsupported,
}

/// One ordered, declarative calculation step. It carries no host behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleStep {
    pub order: u8,
    pub description: &'static str,
}

/// A pure rules-reference record. It documents assumptions rather than asserting native parity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleReference {
    pub id: RuleId,
    pub family: RuleFamily,
    pub evidence: EvidenceStatus,
    pub support: RuleSupport,
    pub assumptions: &'static str,
    pub steps: &'static [RuleStep],
}

/// A bounded lookup result that makes an absent rule explicit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleLookup {
    Found(RuleReference),
    Unsupported {
        family: RuleFamily,
        reason: &'static str,
    },
}

const DAMAGE_STEPS: &[RuleStep] = &[RuleStep {
    order: 1,
    description: "multiply visible card damage by visible hit count",
}];
const COST_STEPS: &[RuleStep] = &[RuleStep {
    order: 1,
    description: "subtract the fixed visible card cost from visible energy",
}];
const BLOCK_STEPS: &[RuleStep] = &[RuleStep {
    order: 1,
    description: "subtract player block from caller-resolved incoming damage, saturating at zero",
}];

const RULES: &[RuleReference] = &[
    RuleReference {
        id: RuleId::NominalCardDamage,
        family: RuleFamily::Damage,
        evidence: EvidenceStatus::SimplifiedModel,
        support: RuleSupport::Conditional,
        assumptions: "No target mitigation, powers, statuses, triggers, rounding, or host effects are represented.",
        steps: DAMAGE_STEPS,
    },
    RuleReference {
        id: RuleId::FixedCardCost,
        family: RuleFamily::ResourceCost,
        evidence: EvidenceStatus::SimplifiedModel,
        support: RuleSupport::Conditional,
        assumptions: "Cost is supplied as a fixed visible value; modifiers and alternative payments are unrepresented.",
        steps: COST_STEPS,
    },
    RuleReference {
        id: RuleId::IncomingDamageAfterBlock,
        family: RuleFamily::Block,
        evidence: EvidenceStatus::SimplifiedModel,
        support: RuleSupport::Conditional,
        assumptions: "Incoming damage is caller-resolved; only player block is applied and no prevention or healing occurs.",
        steps: BLOCK_STEPS,
    },
];

/// Returns the documented record for an exact rule identifier.
#[must_use]
pub fn rules_for(id: RuleId) -> RuleLookup {
    match RULES.iter().copied().find(|rule| rule.id == id) {
        Some(rule) => RuleLookup::Found(rule),
        None => RuleLookup::Unsupported {
            family: RuleFamily::EntityInteraction,
            reason: "no rule record is available",
        },
    }
}

impl RuleFamily {
    /// Returns all records for a mechanic family, or an explicit unsupported outcome.
    #[must_use]
    pub fn lookup(self) -> RuleLookup {
        match RULES.iter().copied().find(|rule| rule.family == self) {
            Some(rule) => RuleLookup::Found(rule),
            None => RuleLookup::Unsupported {
                family: self,
                reason: "this rule family has no evidence-qualified reference",
            },
        }
    }
}
