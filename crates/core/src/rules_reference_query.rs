// SPDX-License-Identifier: MIT

//! Result and query types for bounded rules-reference lookup.

use super::rules_reference::{RuleContext, RuleReference};
use super::rules_reference_ids::RuleId;
use super::rules_reference_vocab::{EntityReference, RuleFamily};

/// Result of an exact rule-ID lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleLookup {
    Found(RuleReference),
    Unsupported {
        family: RuleFamily,
        reason: &'static str,
    },
}

/// A bounded set returned by a family/entity query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleMatches {
    pub rules: &'static [RuleReference],
}

impl RuleMatches {
    /// Returns records in this bounded result.
    #[must_use]
    pub const fn as_slice(self) -> &'static [RuleReference] {
        self.rules
    }

    /// Returns the number of records in this result.
    #[must_use]
    pub const fn len(self) -> usize {
        self.rules.len()
    }

    /// Returns whether no records matched.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.rules.is_empty()
    }
}

/// Collection lookup status; conditional results are never complete game claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleCollectionLookup {
    Found(RuleMatches),
    Conditional {
        matches: RuleMatches,
        reason: &'static str,
    },
    Unsupported {
        family: RuleFamily,
        reason: &'static str,
    },
}

impl RuleCollectionLookup {
    /// Returns whether the result is conditional rather than unsupported.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(self, Self::Conditional { .. })
    }

    /// Returns the bounded matches, if this result carries any.
    #[must_use]
    pub const fn matches(self) -> Option<RuleMatches> {
        match self {
            Self::Found(matches) | Self::Conditional { matches, .. } => Some(matches),
            Self::Unsupported { .. } => None,
        }
    }

    /// Returns the reason carried by a conditional or unsupported result.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Found(_) => "",
            Self::Conditional { reason, .. } | Self::Unsupported { reason, .. } => reason,
        }
    }
}

/// Query by one indexed dimension.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RuleQuery<'a> {
    pub rule_id: Option<RuleId>,
    pub mechanic: Option<RuleFamily>,
    pub entity: Option<EntityReference<'a>>,
    pub context: Option<RuleContext<'a>>,
}

impl<'a> RuleQuery<'a> {
    /// Creates an exact rule query.
    #[must_use]
    pub const fn by_id(rule_id: RuleId) -> Self {
        Self {
            rule_id: Some(rule_id),
            mechanic: None,
            entity: None,
            context: None,
        }
    }

    /// Creates a mechanic query.
    #[must_use]
    pub const fn by_mechanic(mechanic: RuleFamily) -> Self {
        Self {
            rule_id: None,
            mechanic: Some(mechanic),
            entity: None,
            context: None,
        }
    }

    /// Creates an entity query.
    #[must_use]
    pub const fn by_entity(entity: EntityReference<'a>) -> Self {
        Self {
            rule_id: None,
            mechanic: None,
            entity: Some(entity),
            context: None,
        }
    }

    /// Adds an applicability context.
    #[must_use]
    pub const fn in_context(mut self, context: RuleContext<'a>) -> Self {
        self.context = Some(context);
        self
    }
}

/// Family inventory status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleCoverageStatus {
    Partial,
    Unmodeled,
}

/// Explicit record of modeled and remaining family coverage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleFamilyCoverage {
    pub family: RuleFamily,
    pub status: RuleCoverageStatus,
    pub rules: &'static [RuleReference],
    pub unmodeled: &'static str,
}
