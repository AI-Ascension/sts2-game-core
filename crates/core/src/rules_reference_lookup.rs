// SPDX-License-Identifier: MIT

use super::rules_reference::{
    EntityKind, EntityReference, Mechanic, RuleContext, RuleFamily, RuleId, RuleReference,
    RuleSupport,
};
use super::rules_reference_catalog as catalog;
use super::rules_reference_query::{ALL_FAMILIES, FAMILY_COVERAGE};
use super::rules_reference_query::{
    RuleCollectionLookup, RuleCoverageStatus, RuleFamilyCoverage, RuleLookup, RuleMatches,
    RuleQuery,
};

/// Returns one exact record, or an explicit unsupported result.
#[must_use]
pub fn rules_for(id: RuleId) -> RuleLookup {
    match catalog::RULES.iter().copied().find(|rule| rule.id == id) {
        Some(rule) => RuleLookup::Found(rule),
        None => RuleLookup::Unsupported {
            family: RuleFamily::EntityInteraction,
            reason: "no rule record is available",
        },
    }
}

/// Returns the bounded record inventory.
#[must_use]
pub const fn rule_inventory() -> &'static [RuleReference] {
    catalog::RULES
}

/// Returns a family row with remaining exclusions.
#[must_use]
pub fn coverage_for(family: RuleFamily) -> RuleFamilyCoverage {
    if let Some(row) = FAMILY_COVERAGE.iter().find(|row| row.family == family) {
        return *row;
    }
    RuleFamilyCoverage {
        family,
        status: RuleCoverageStatus::Unmodeled,
        rules: catalog::NO_IDS,
        unmodeled: "no evidence-qualified record is available",
    }
}

/// Returns every family tracked by this inventory.
#[must_use]
pub const fn coverage_inventory() -> &'static [RuleFamily] {
    ALL_FAMILIES
}

/// Looks up all records indexed by one mechanic.
#[must_use]
pub fn rules_for_mechanic(mechanic: Mechanic) -> RuleCollectionLookup {
    match mechanic {
        RuleFamily::Damage => RuleCollectionLookup::Conditional {
            matches: RuleMatches {
                rules: catalog::DAMAGE_RULES,
            },
            reason: "damage records omit mitigation, triggers, and cross-family combinations",
        },
        RuleFamily::ResourceCost => RuleCollectionLookup::Conditional {
            matches: RuleMatches {
                rules: catalog::COST_RULES,
            },
            reason: "resource-cost records omit modifiers and alternative payments",
        },
        RuleFamily::Block => RuleCollectionLookup::Conditional {
            matches: RuleMatches {
                rules: catalog::BLOCK_RULES,
            },
            reason: "block records omit prevention, healing, and death replacement",
        },
        family => RuleCollectionLookup::Unsupported {
            family,
            reason: "this mechanic has no evidence-qualified record in the bounded inventory",
        },
    }
}

/// Looks up records related to one card, relic, status, or rule key.
#[must_use]
pub fn rules_for_entity(entity: EntityReference<'_>) -> RuleCollectionLookup {
    match (entity.kind, entity.id) {
        (EntityKind::Card, "card.definition") => conditional(
            catalog::CARD_RULES,
            "card references cover only nominal damage and fixed cost",
        ),
        (EntityKind::Card, "synthetic.card.multi_hit")
        | (EntityKind::Relic, "synthetic.relic.flat_damage")
        | (EntityKind::Status, "synthetic.status.multiplier") => conditional(
            catalog::SYNTHETIC_RULES,
            "synthetic interaction coverage is conditional and has no native evidence",
        ),
        (EntityKind::Rule, "damage.nominal_card") => conditional(
            catalog::NOMINAL_RULES,
            "the referenced rule remains a simplified conditional model",
        ),
        (EntityKind::Rule, "resource.fixed_card_cost") => conditional(
            catalog::FIXED_COST_RULES,
            "the referenced rule remains a simplified conditional model",
        ),
        (EntityKind::Rule, "block.incoming_after_block") => conditional(
            catalog::INCOMING_BLOCK_RULES,
            "the referenced rule remains a simplified conditional model",
        ),
        (EntityKind::Rule, "damage.synthetic_modifier_ordering") => conditional(
            catalog::SYNTHETIC_ORDERING_RULES,
            "the referenced rule is synthetic and has no native evidence",
        ),
        (EntityKind::Player, "player") => conditional(
            catalog::BLOCK_RULES,
            "player references cover only incoming damage after visible block",
        ),
        _ => unsupported_entity(entity.kind),
    }
}

fn conditional(rules: &'static [RuleReference], reason: &'static str) -> RuleCollectionLookup {
    RuleCollectionLookup::Conditional {
        matches: RuleMatches { rules },
        reason,
    }
}

fn unsupported_entity(kind: EntityKind) -> RuleCollectionLookup {
    let family = match kind {
        EntityKind::Card => RuleFamily::CardInteraction,
        EntityKind::Relic => RuleFamily::RelicInteraction,
        EntityKind::Potion => RuleFamily::PotionInteraction,
        EntityKind::Status => RuleFamily::StatusInteraction,
        EntityKind::Rule | EntityKind::Enemy | EntityKind::Player => RuleFamily::EntityInteraction,
    };
    RuleCollectionLookup::Unsupported {
        family,
        reason: "no rule record names this entity reference",
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

fn unsupported(reason: &'static str) -> RuleCollectionLookup {
    RuleCollectionLookup::Unsupported {
        family: RuleFamily::EntityInteraction,
        reason,
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
    let rules = match rule_id {
        RuleId::NominalCardDamage => catalog::NOMINAL_RULES,
        RuleId::FixedCardCost => catalog::FIXED_COST_RULES,
        RuleId::IncomingDamageAfterBlock => catalog::INCOMING_BLOCK_RULES,
        RuleId::SyntheticModifierOrdering => catalog::SYNTHETIC_ORDERING_RULES,
    };
    let matches = RuleMatches { rules };
    match rule.support {
        RuleSupport::Supported => RuleCollectionLookup::Found(matches),
        RuleSupport::Conditional => RuleCollectionLookup::Conditional {
            matches,
            reason: rule.assumptions,
        },
        RuleSupport::Unsupported => RuleCollectionLookup::Unsupported {
            family: rule.family,
            reason: "the rule record is marked unsupported",
        },
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
