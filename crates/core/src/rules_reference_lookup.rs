// SPDX-License-Identifier: MIT

//! Bounded lookup over the pure rules-reference inventory.
//!
//! A query answers only within established coverage. An unmodeled family returns `Unsupported`,
//! a family with remaining exclusions returns `Conditional`, and nothing converts an unknown
//! request into a zero, an empty description, or a successful partial result.

use super::rules_reference::{RuleContext, RuleReference, RuleSupport};
use super::rules_reference_catalog as catalog;
use super::rules_reference_coverage as coverage;
use super::rules_reference_coverage::UnmodeledCombination;
use super::rules_reference_ids::RuleId;
use super::rules_reference_index as index;
use super::rules_reference_query::{
    RuleCollectionLookup, RuleFamilyCoverage, RuleLookup, RuleMatches, RuleQuery,
};
use super::rules_reference_vocab::{EntityKind, EntityReference, RuleFamily};

/// Returns one exact record, or an explicit unsupported result.
#[must_use]
pub fn rules_for(id: RuleId) -> RuleLookup {
    match catalog::find(id) {
        Some(rule) => RuleLookup::Found(rule),
        None => RuleLookup::Unsupported {
            family: id.family(),
            reason: "no rule record is available",
        },
    }
}

/// Returns the bounded record inventory.
#[must_use]
pub const fn rule_inventory() -> &'static [RuleReference] {
    catalog::RULES
}

/// Returns a family row with its remaining exclusions.
#[must_use]
pub fn coverage_for(family: RuleFamily) -> RuleFamilyCoverage {
    coverage::coverage_row(family).unwrap_or(RuleFamilyCoverage {
        family,
        status: super::rules_reference_query::RuleCoverageStatus::Unmodeled,
        rules: &[],
        unmodeled: "no evidence-qualified record is available",
    })
}

/// Returns every family tracked by this inventory.
#[must_use]
pub const fn coverage_inventory() -> &'static [RuleFamily] {
    coverage::ALL_FAMILIES
}

/// Returns the explicitly unmodeled combinations recorded for one family.
#[must_use]
pub fn unmodeled_combinations(family: RuleFamily) -> &'static [UnmodeledCombination] {
    coverage::exclusions_for_family(family)
}

/// Returns the unmodeled combinations declared by one record.
#[must_use]
pub fn unmodeled_for(id: RuleId) -> Option<&'static [&'static str]> {
    catalog::find(id).map(|rule| rule.unmodeled)
}

/// Looks up every record indexed by one mechanic.
#[must_use]
pub fn rules_for_mechanic(mechanic: RuleFamily) -> RuleCollectionLookup {
    collection(catalog::family_slice(mechanic), mechanic)
}

/// Looks up records by one card, relic, potion, status, enemy, player, or rule reference.
#[must_use]
pub fn rules_for_entity(entity: EntityReference<'_>) -> RuleCollectionLookup {
    if entity.kind == EntityKind::Rule {
        return match catalog::RULES
            .iter()
            .copied()
            .find(|rule| rule.id.matches_key(entity.id))
        {
            Some(rule) => rule_result(rule),
            None => RuleCollectionLookup::Unsupported {
                family: RuleFamily::EntityInteraction,
                reason: "no rule record carries this rule reference",
            },
        };
    }
    match index::find(entity) {
        Some(rules) => collection(rules, family_for_entity(entity.kind)),
        None => RuleCollectionLookup::Unsupported {
            family: family_for_entity(entity.kind),
            reason: "no rule record names this entity reference",
        },
    }
}

/// Executes a bounded query. Compound and context-only queries are unsupported.
#[must_use]
pub fn lookup_rules(query: RuleQuery<'_>) -> RuleCollectionLookup {
    let dimensions = u8::from(query.rule_id.is_some())
        + u8::from(query.mechanic.is_some())
        + u8::from(query.entity.is_some());
    if dimensions > 1 {
        return unsupported("compound rule queries are outside the bounded index");
    }
    if let Some(rule_id) = query.rule_id {
        return lookup_rule_id(rule_id, query.context);
    }
    if let Some(mechanic) = query.mechanic {
        if query.context.is_some() {
            return RuleCollectionLookup::Unsupported {
                family: mechanic,
                reason: "context-filtered mechanic queries are not indexed",
            };
        }
        return rules_for_mechanic(mechanic);
    }
    if let Some(entity) = query.entity {
        if query.context.is_some() {
            return unsupported("context-filtered entity queries are not indexed");
        }
        return rules_for_entity(entity);
    }
    unsupported("a rule ID, mechanic, or entity reference is required")
}

/// Alias for [`lookup_rules`] for callers that use a generic lookup verb.
#[must_use]
pub fn lookup(query: RuleQuery<'_>) -> RuleCollectionLookup {
    lookup_rules(query)
}

fn collection(rules: &'static [RuleReference], family: RuleFamily) -> RuleCollectionLookup {
    if rules.is_empty() {
        return RuleCollectionLookup::Unsupported {
            family,
            reason: family_reason(family),
        };
    }
    let all_supported = rules
        .iter()
        .all(|rule| rule.support == RuleSupport::Supported);
    if all_supported && !coverage::has_unmodeled_combinations(family) {
        return RuleCollectionLookup::Found(RuleMatches { rules });
    }
    RuleCollectionLookup::Conditional {
        matches: RuleMatches { rules },
        reason: family_reason(family),
    }
}

fn rule_result(rule: RuleReference) -> RuleCollectionLookup {
    let Some(rules) = catalog::slice_of(rule.id) else {
        return unsupported("no rule record is available");
    };
    let matches = RuleMatches { rules };
    match rule.support {
        RuleSupport::Unsupported => RuleCollectionLookup::Unsupported {
            family: rule.family,
            reason: "the rule record is marked unsupported",
        },
        RuleSupport::Conditional => RuleCollectionLookup::Conditional {
            matches,
            reason: rule.assumptions,
        },
        RuleSupport::Supported => {
            if coverage::has_unmodeled_combinations(rule.family) {
                RuleCollectionLookup::Conditional {
                    matches,
                    reason: family_reason(rule.family),
                }
            } else {
                RuleCollectionLookup::Found(matches)
            }
        }
    }
}

fn lookup_rule_id(rule_id: RuleId, context: Option<RuleContext<'_>>) -> RuleCollectionLookup {
    let RuleLookup::Found(rule) = rules_for(rule_id) else {
        return unsupported("no rule record is available");
    };
    if let Some(context) = context
        && !rule.applies_to(context)
    {
        return RuleCollectionLookup::Unsupported {
            family: rule.family,
            reason: "rule applicability excludes the supplied content/build/mode",
        };
    }
    rule_result(rule)
}

fn family_reason(family: RuleFamily) -> &'static str {
    coverage::coverage_row(family).map_or("coverage is conditional", |row| row.unmodeled)
}

fn family_for_entity(kind: EntityKind) -> RuleFamily {
    match kind {
        EntityKind::Card => RuleFamily::CardInteraction,
        EntityKind::Relic => RuleFamily::RelicInteraction,
        EntityKind::Potion => RuleFamily::PotionInteraction,
        EntityKind::Status => RuleFamily::StatusInteraction,
        EntityKind::Enemy | EntityKind::Player | EntityKind::Rule => RuleFamily::EntityInteraction,
    }
}

fn unsupported(reason: &'static str) -> RuleCollectionLookup {
    RuleCollectionLookup::Unsupported {
        family: RuleFamily::EntityInteraction,
        reason,
    }
}

impl RuleFamily {
    /// Preserves the original first-record family lookup API.
    #[must_use]
    pub fn lookup(self) -> RuleLookup {
        match catalog::RULES
            .iter()
            .copied()
            .find(|rule| rule.family == self)
        {
            Some(rule) => RuleLookup::Found(rule),
            None => RuleLookup::Unsupported {
                family: self,
                reason: "this rule family has no evidence-qualified reference",
            },
        }
    }

    /// Returns every bounded record for this mechanic.
    #[must_use]
    pub fn lookup_all(self) -> RuleCollectionLookup {
        rules_for_mechanic(self)
    }
}
