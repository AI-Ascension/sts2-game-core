// SPDX-License-Identifier: MIT

use super::rules_reference_data as data;
use super::{
    EntityReference, EvidenceStatus, ExpiryRule, RoundingRule, RuleApplicability, RuleFamily,
    RuleId, RuleInput, RuleReference, RuleStep, RuleSupport, SourceStatus, StackRule,
    TargetingRule,
};

#[derive(Clone, Copy)]
struct Semantics {
    rounding: RoundingRule,
    targeting: TargetingRule,
    stacking: StackRule,
    expiry: ExpiryRule,
    entities: &'static [EntityReference<'static>],
    related_rules: &'static [RuleId],
}
#[derive(Clone, Copy)]
struct Provenance {
    evidence: EvidenceStatus,
    source: SourceStatus,
    source_ref: &'static str,
    support: RuleSupport,
    assumptions: &'static str,
}
const fn reference(
    id: RuleId,
    family: RuleFamily,
    applicability: RuleApplicability,
    inputs: &'static [RuleInput],
    steps: &'static [RuleStep],
    semantics: Semantics,
    provenance: Provenance,
) -> RuleReference {
    RuleReference {
        version: super::RULES_REFERENCE_VERSION,
        id,
        family,
        mechanic: family,
        applicability,
        inputs,
        steps,
        rounding: semantics.rounding,
        targeting: semantics.targeting,
        stacking: semantics.stacking,
        expiry: semantics.expiry,
        entities: semantics.entities,
        related_rules: semantics.related_rules,
        evidence: provenance.evidence,
        source: provenance.source,
        source_ref: provenance.source_ref,
        support: provenance.support,
        assumptions: provenance.assumptions,
    }
}
const NOMINAL_SEMANTICS: Semantics = Semantics {
    rounding: RoundingRule::NotRepresented,
    targeting: TargetingRule::SingleEnemy,
    stacking: StackRule::NotRepresented,
    expiry: ExpiryRule::Immediate,
    entities: data::CARD_ENTITIES,
    related_rules: data::NOMINAL_RELATED,
};
const COST_SEMANTICS: Semantics = Semantics {
    rounding: RoundingRule::None,
    targeting: TargetingRule::None,
    stacking: StackRule::NotApplicable,
    expiry: ExpiryRule::Immediate,
    entities: data::CARD_ENTITIES,
    related_rules: data::NO_RULES,
};
const BLOCK_SEMANTICS: Semantics = Semantics {
    rounding: RoundingRule::None,
    targeting: TargetingRule::SelfPlayer,
    stacking: StackRule::NotApplicable,
    expiry: ExpiryRule::Immediate,
    entities: data::BLOCK_ENTITIES,
    related_rules: data::NO_RULES,
};
const SYNTHETIC_SEMANTICS: Semantics = Semantics {
    rounding: RoundingRule::Floor,
    targeting: TargetingRule::SingleEnemy,
    stacking: StackRule::Additive,
    expiry: ExpiryRule::Immediate,
    entities: data::SYNTHETIC_ENTITIES,
    related_rules: data::SYNTHETIC_RELATED,
};
const SIMPLIFIED_DAMAGE_PROVENANCE: Provenance = Provenance {
    evidence: EvidenceStatus::SimplifiedModel,
    source: SourceStatus::Synthetic,
    source_ref: "crates/core/src/calculators.rs@9297923",
    support: RuleSupport::Conditional,
    assumptions: "This is per-target damage only; all-enemy aggregation, mitigation, powers, statuses, triggers, and rounding are unrepresented.",
};
const SIMPLIFIED_COST_PROVENANCE: Provenance = Provenance {
    evidence: EvidenceStatus::SimplifiedModel,
    source: SourceStatus::Synthetic,
    source_ref: "crates/core/src/calculators.rs@9297923",
    support: RuleSupport::Conditional,
    assumptions: "Cost is supplied as a fixed visible value; cost modifiers, alternative payments, and triggered costs are unrepresented.",
};
const SIMPLIFIED_BLOCK_PROVENANCE: Provenance = Provenance {
    evidence: EvidenceStatus::SimplifiedModel,
    source: SourceStatus::Synthetic,
    source_ref: "crates/core/src/calculators.rs@9297923",
    support: RuleSupport::Conditional,
    assumptions: "Incoming damage is caller-resolved; only player block is applied, with no prevention, healing, death replacement, or trigger.",
};
const SYNTHETIC_PROVENANCE: Provenance = Provenance {
    evidence: EvidenceStatus::SyntheticFixture,
    source: SourceStatus::Synthetic,
    source_ref: "synthetic.rules-reference.v1",
    support: RuleSupport::Conditional,
    assumptions: "Synthetic interaction only; this record is not native or host-parity evidence.",
};
const NOMINAL_CARD_DAMAGE: RuleReference = reference(
    RuleId::NominalCardDamage,
    RuleFamily::Damage,
    data::ANY_APPLICABILITY,
    data::DAMAGE_INPUTS,
    data::DAMAGE_STEPS,
    NOMINAL_SEMANTICS,
    SIMPLIFIED_DAMAGE_PROVENANCE,
);
const FIXED_CARD_COST: RuleReference = reference(
    RuleId::FixedCardCost,
    RuleFamily::ResourceCost,
    data::ANY_APPLICABILITY,
    data::COST_INPUTS,
    data::COST_STEPS,
    COST_SEMANTICS,
    SIMPLIFIED_COST_PROVENANCE,
);
const INCOMING_DAMAGE_AFTER_BLOCK: RuleReference = reference(
    RuleId::IncomingDamageAfterBlock,
    RuleFamily::Block,
    data::ANY_APPLICABILITY,
    data::BLOCK_INPUTS,
    data::BLOCK_STEPS,
    BLOCK_SEMANTICS,
    SIMPLIFIED_BLOCK_PROVENANCE,
);
const SYNTHETIC_MODIFIER_ORDERING: RuleReference = reference(
    RuleId::SyntheticModifierOrdering,
    RuleFamily::Damage,
    data::SYNTHETIC_APPLICABILITY,
    data::SYNTHETIC_DAMAGE_INPUTS,
    data::SYNTHETIC_DAMAGE_STEPS,
    SYNTHETIC_SEMANTICS,
    SYNTHETIC_PROVENANCE,
);

pub(crate) const RULES: &[RuleReference] = &[
    NOMINAL_CARD_DAMAGE,
    FIXED_CARD_COST,
    INCOMING_DAMAGE_AFTER_BLOCK,
    SYNTHETIC_MODIFIER_ORDERING,
];
pub(crate) const DAMAGE_RULES: &[RuleReference] =
    &[NOMINAL_CARD_DAMAGE, SYNTHETIC_MODIFIER_ORDERING];
pub(crate) const COST_RULES: &[RuleReference] = &[FIXED_CARD_COST];
pub(crate) const BLOCK_RULES: &[RuleReference] = &[INCOMING_DAMAGE_AFTER_BLOCK];
pub(crate) const CARD_RULES: &[RuleReference] = &[NOMINAL_CARD_DAMAGE, FIXED_CARD_COST];
pub(crate) const SYNTHETIC_RULES: &[RuleReference] = &[SYNTHETIC_MODIFIER_ORDERING];
pub(crate) const NOMINAL_RULES: &[RuleReference] = &[NOMINAL_CARD_DAMAGE];
pub(crate) const FIXED_COST_RULES: &[RuleReference] = &[FIXED_CARD_COST];
pub(crate) const INCOMING_BLOCK_RULES: &[RuleReference] = &[INCOMING_DAMAGE_AFTER_BLOCK];
pub(crate) const SYNTHETIC_ORDERING_RULES: &[RuleReference] = &[SYNTHETIC_MODIFIER_ORDERING];
pub(crate) const DAMAGE_IDS: &[RuleId] =
    &[RuleId::NominalCardDamage, RuleId::SyntheticModifierOrdering];
pub(crate) const COST_IDS: &[RuleId] = &[RuleId::FixedCardCost];
pub(crate) const BLOCK_IDS: &[RuleId] = &[RuleId::IncomingDamageAfterBlock];
pub(crate) const NO_IDS: &[RuleId] = &[];
