// SPDX-License-Identifier: MIT

//! Public record types for the pure, versioned rules-reference model.

use super::rules_reference_ids::RuleId;
use super::rules_reference_vocab::{
    EntityReference, ExpiryRule, RoundingRule, RuleFamily, RuleInput, RuleStep, StackRule,
    TargetingRule,
};

/// Version of the core-owned rules-reference inventory.
pub const RULES_REFERENCE_VERSION: u16 = 2;
/// Bound on one collection lookup. No single query can return more than the whole inventory.
pub const MAX_RULE_MATCHES: usize = 16;

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

/// Content applicability scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentScope {
    Any,
    Named(&'static str),
}

/// Build applicability scope.
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

/// One pure rules-reference record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleReference {
    pub version: u16,
    pub id: RuleId,
    pub family: RuleFamily,
    pub mechanic: RuleFamily,
    pub applicability: RuleApplicability,
    pub inputs: &'static [RuleInput],
    pub steps: &'static [RuleStep],
    pub rounding: RoundingRule,
    pub targeting: TargetingRule,
    pub stacking: StackRule,
    pub expiry: ExpiryRule,
    pub entities: &'static [EntityReference<'static>],
    pub related_rules: &'static [RuleId],
    pub unmodeled: &'static [&'static str],
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

    /// Returns whether this record claims a host comparison source.
    #[must_use]
    pub fn claims_native_source(self) -> bool {
        self.source == SourceStatus::NativeComparison
    }
}
