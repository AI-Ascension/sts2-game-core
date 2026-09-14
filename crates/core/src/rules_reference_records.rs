// SPDX-License-Identifier: MIT

//! Construction helpers and shared applicability for rules-reference records.

use super::rules_reference::{
    BuildScope, ContentScope, EvidenceStatus, GameMode, RuleApplicability, RuleReference,
    RuleSupport, SourceStatus,
};
use super::rules_reference_ids::RuleId;
use super::rules_reference_vocab::{
    EntityKind, EntityReference, ExpiryRule, RoundingRule, RuleFamily, RuleInput, RuleOperation,
    RuleStep, RuleStepKind, RuleUnit, StackRule, TargetingRule,
};

pub(crate) const ANY_MODES: &[GameMode] = &[GameMode::Any];
pub(crate) const SOLO_MODE: &[GameMode] = &[GameMode::Solo];

/// Applicability for records that mirror the simplified calculators.
pub(crate) const ANY_APPLICABILITY: RuleApplicability = RuleApplicability {
    content: ContentScope::Any,
    build: BuildScope::Any,
    modes: ANY_MODES,
};

/// Applicability for project-owned synthetic semantics.
pub(crate) const SYNTHETIC_APPLICABILITY: RuleApplicability = RuleApplicability {
    content: ContentScope::Named("synthetic-rules-v1"),
    build: BuildScope::Named("synthetic-build-v1"),
    modes: SOLO_MODE,
};

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
pub(crate) const POTION_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Potion,
    id: "synthetic.potion",
};
pub(crate) const ENEMY_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Enemy,
    id: "enemy.definition",
};
pub(crate) const PLAYER_ENTITY: EntityReference<'static> = EntityReference {
    kind: EntityKind::Player,
    id: "player",
};

pub(crate) const NO_RULES: &[RuleId] = &[];

/// Shared provenance used by records that mirror the simplified calculators.
pub(crate) const SIMPLIFIED_SOURCE: &str = "crates/core/src/calculators.rs@9297923";
/// Stable key for project-owned synthetic semantics.
pub(crate) const SYNTHETIC_SOURCE: &str = "synthetic.rules-reference.v1";

/// Declarative lifecycle semantics attached to a record.
#[derive(Clone, Copy)]
pub(crate) struct Semantics {
    pub(crate) rounding: RoundingRule,
    pub(crate) targeting: TargetingRule,
    pub(crate) stacking: StackRule,
    pub(crate) expiry: ExpiryRule,
    pub(crate) entities: &'static [EntityReference<'static>],
    pub(crate) related_rules: &'static [RuleId],
}

/// Evidence, support, and explicit unmodeled combinations for a record.
#[derive(Clone, Copy)]
pub(crate) struct Provenance {
    pub(crate) evidence: EvidenceStatus,
    pub(crate) source: SourceStatus,
    pub(crate) source_ref: &'static str,
    pub(crate) support: RuleSupport,
    pub(crate) assumptions: &'static str,
    pub(crate) unmodeled: &'static [&'static str],
}

pub(crate) const fn input(
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

pub(crate) const fn step(
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

pub(crate) const fn semantics(
    rounding: RoundingRule,
    targeting: TargetingRule,
    stacking: StackRule,
    expiry: ExpiryRule,
    entities: &'static [EntityReference<'static>],
    related_rules: &'static [RuleId],
) -> Semantics {
    Semantics {
        rounding,
        targeting,
        stacking,
        expiry,
        entities,
        related_rules,
    }
}

pub(crate) const fn provenance(
    evidence: EvidenceStatus,
    source: SourceStatus,
    source_ref: &'static str,
    support: RuleSupport,
    assumptions: &'static str,
    unmodeled: &'static [&'static str],
) -> Provenance {
    Provenance {
        evidence,
        source,
        source_ref,
        support,
        assumptions,
        unmodeled,
    }
}

pub(crate) const fn reference(
    id: RuleId,
    family: RuleFamily,
    applicability: RuleApplicability,
    inputs: &'static [RuleInput],
    steps: &'static [RuleStep],
    semantics: Semantics,
    provenance: Provenance,
) -> RuleReference {
    RuleReference {
        version: super::rules_reference::RULES_REFERENCE_VERSION,
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
        unmodeled: provenance.unmodeled,
        evidence: provenance.evidence,
        source: provenance.source,
        source_ref: provenance.source_ref,
        support: provenance.support,
        assumptions: provenance.assumptions,
    }
}
