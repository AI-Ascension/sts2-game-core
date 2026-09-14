// SPDX-License-Identifier: MIT

use sts2_game_core::{
    BLOCK_EXPIRY_CLAMPED_FIXTURE, BLOCK_EXPIRY_FIXTURE, CardSpec, CardTarget,
    CombatCalculationState, EnemyFacts, EntityKind, EntityReference, EvidenceStatus, ExpiryRule,
    FixtureClass, GameMode, MAX_RULE_MATCHES, ORDERED_FIXTURES, RoundingRule, RuleCollectionLookup,
    RuleCoverageStatus, RuleFamily, RuleId, RuleLookup, RuleQuery, RuleStepKind, RuleSupport,
    SYNTHETIC_MODIFIER_FIXTURE, SourceStatus, StackRule, TargetDomain, TargetingRule, coverage_for,
    coverage_inventory, exact_card_damage, lookup_rules, rule_inventory, rules_for,
    rules_for_entity, rules_for_mechanic,
};

#[test]
fn documented_calculators_are_never_promoted_to_host_parity() {
    let lookup = rules_for(RuleId::NominalCardDamage);
    assert!(matches!(lookup, RuleLookup::Found(_)));
    let RuleLookup::Found(rule) = lookup else {
        return;
    };
    assert_eq!(rule.evidence, EvidenceStatus::SimplifiedModel);
    assert_eq!(rule.source, SourceStatus::Synthetic);
    assert_eq!(rule.source_ref, "crates/core/src/calculators.rs@9297923");
    assert_eq!(rule.support, RuleSupport::Conditional);
    assert_eq!(rule.rounding, RoundingRule::NotRepresented);
    assert_eq!(rule.targeting, TargetingRule::SingleEnemy);
    assert_eq!(rule.stacking, StackRule::NotRepresented);
    assert_eq!(rule.expiry, ExpiryRule::Immediate);
    assert_eq!(rule.inputs[0].unit, sts2_game_core::RuleUnit::Damage);
    assert_eq!(rule.steps[0].order, 1);
    assert_eq!(rule.steps[0].kind, RuleStepKind::Calculation);
    assert_eq!(
        rule.steps[0].description,
        "multiply visible card damage by visible hit count for one target"
    );
    assert!(rule.assumptions.contains("per-target damage only"));
}

#[test]
fn project_owned_fixtures_reproduce_their_declared_expectations() {
    assert!(!ORDERED_FIXTURES.is_empty());
    for fixture in ORDERED_FIXTURES {
        assert!(
            matches!(rules_for(fixture.rule), RuleLookup::Found(_)),
            "{:?} fixture names no record",
            fixture.rule
        );
        assert_eq!(fixture.class, FixtureClass::ConfirmedSyntheticRule);
        assert!(
            fixture.matches_expected(),
            "{:?} fixture does not reproduce its declared value",
            fixture.rule
        );
    }
}

#[test]
fn confirmed_records_are_backed_by_a_project_owned_fixture() {
    let mut confirmed = 0;
    for rule in rule_inventory() {
        if rule.evidence != EvidenceStatus::Confirmed {
            continue;
        }
        confirmed += 1;
        assert_eq!(rule.support, RuleSupport::Supported, "{:?}", rule.id);
        assert!(
            ORDERED_FIXTURES
                .iter()
                .any(|fixture| fixture.rule == rule.id),
            "{:?} claims a project-owned fixture that is not declared",
            rule.id
        );
    }
    assert_eq!(confirmed, 11);
}

#[test]
fn declared_block_expiry_settles_the_remaining_block_at_zero() {
    assert_eq!(BLOCK_EXPIRY_FIXTURE.rule, RuleId::BlockExpiryAtTurnStart);
    assert!(BLOCK_EXPIRY_FIXTURE.matches_expected());
    assert_eq!(BLOCK_EXPIRY_FIXTURE.evaluate(), Some(5 - 3));
    assert!(BLOCK_EXPIRY_CLAMPED_FIXTURE.matches_expected());
    assert_eq!(
        BLOCK_EXPIRY_CLAMPED_FIXTURE.evaluate(),
        Some(2_u64.saturating_sub(5))
    );
    assert_eq!(BLOCK_EXPIRY_CLAMPED_FIXTURE.evaluate(), Some(0));
}

#[test]
fn unmodeled_family_and_unknown_entity_queries_are_explicitly_unsupported() {
    assert!(matches!(
        RuleFamily::EntityInteraction.lookup(),
        RuleLookup::Unsupported { .. }
    ));
    assert_eq!(
        coverage_for(RuleFamily::EntityInteraction).status,
        RuleCoverageStatus::Unmodeled
    );
    assert!(matches!(
        rules_for_mechanic(RuleFamily::EntityInteraction),
        RuleCollectionLookup::Unsupported { .. }
    ));
    for family in [
        RuleFamily::Heal,
        RuleFamily::CardMovement,
        RuleFamily::TurnTiming,
        RuleFamily::Acquisition,
        RuleFamily::DifficultyScaling,
        RuleFamily::CoopScaling,
        RuleFamily::CardInteraction,
        RuleFamily::RelicInteraction,
        RuleFamily::PotionInteraction,
        RuleFamily::StatusInteraction,
    ] {
        assert_eq!(coverage_for(family).status, RuleCoverageStatus::Partial);
        assert!(rules_for_mechanic(family).matches().is_some());
    }
}

#[test]
fn synthetic_interaction_fixture_verifies_order_and_rounding() {
    let lookup = rules_for(RuleId::SyntheticModifierOrdering);
    assert!(matches!(lookup, RuleLookup::Found(_)));
    let RuleLookup::Found(rule) = lookup else {
        return;
    };
    assert_eq!(rule.evidence, EvidenceStatus::SyntheticFixture);
    assert_eq!(rule.source, SourceStatus::Synthetic);
    assert_eq!(rule.source_ref, "synthetic.rules-reference.v1");
    assert_eq!(rule.support, RuleSupport::Conditional);
    assert_eq!(rule.rounding, RoundingRule::Floor);
    assert_eq!(rule.stacking, StackRule::Additive);
    assert_eq!(rule.expiry, ExpiryRule::Immediate);
    assert_eq!(rule.steps.len(), 5);
    assert_eq!(rule.steps[0].kind, RuleStepKind::Calculation);
    assert_eq!(
        rule.steps[1].operation,
        sts2_game_core::RuleOperation::Multiply
    );
    assert_eq!(rule.steps[2].kind, RuleStepKind::Rounding);
    assert_eq!(
        rule.steps[3].operation,
        sts2_game_core::RuleOperation::Multiply
    );
    assert_eq!(
        SYNTHETIC_MODIFIER_FIXTURE.ordered_damage(),
        Some(SYNTHETIC_MODIFIER_FIXTURE.expected_damage)
    );
    let wrong_order = (SYNTHETIC_MODIFIER_FIXTURE.base_damage
        + SYNTHETIC_MODIFIER_FIXTURE.flat_bonus)
        * u32::from(SYNTHETIC_MODIFIER_FIXTURE.hits)
        * SYNTHETIC_MODIFIER_FIXTURE.multiplier_numerator
        / SYNTHETIC_MODIFIER_FIXTURE.multiplier_denominator;
    assert_ne!(wrong_order, SYNTHETIC_MODIFIER_FIXTURE.expected_damage);
    let simple_card = CardSpec {
        card_id: 1,
        cost: 0,
        damage: 5,
        hits: 2,
        block: 0,
        target: TargetDomain::SingleEnemy,
    };
    let simple_state = CombatCalculationState {
        player_hp: 1,
        max_player_hp: 1,
        player_block: 0,
        incoming_damage: 0,
        energy: 0,
        enemies: vec![EnemyFacts {
            enemy_id: 1,
            hp: 20,
            max_hp: 20,
        }],
    };
    let simple = exact_card_damage(&simple_card, &simple_state, CardTarget::Enemy(1));
    assert_eq!(simple.map(|damage| damage.damage), Ok(10));
    assert_ne!(
        simple.map(|damage| damage.damage),
        Ok(SYNTHETIC_MODIFIER_FIXTURE.expected_damage)
    );
}

#[test]
fn applicability_and_entity_queries_are_bounded() {
    let lookup = rules_for(RuleId::SyntheticModifierOrdering);
    assert!(matches!(lookup, RuleLookup::Found(_)));
    let RuleLookup::Found(synthetic) = lookup else {
        return;
    };
    assert!(synthetic.applies_to(sts2_game_core::RuleContext {
        content: "synthetic-rules-v1",
        build: "synthetic-build-v1",
        mode: GameMode::Solo,
    }));
    assert!(!synthetic.applies_to(sts2_game_core::RuleContext {
        content: "other-content",
        build: "synthetic-build-v1",
        mode: GameMode::Solo,
    }));
    let damage = rules_for_mechanic(RuleFamily::Damage);
    assert!(matches!(damage, RuleCollectionLookup::Conditional { .. }));
    let RuleCollectionLookup::Conditional { matches, .. } = damage else {
        return;
    };
    assert!(matches.len() <= MAX_RULE_MATCHES);
    assert!(
        rules_for_entity(EntityReference {
            kind: EntityKind::Relic,
            id: "synthetic.relic.flat_damage",
        })
        .is_conditional()
    );
    assert!(matches!(
        rules_for_entity(EntityReference {
            kind: EntityKind::Potion,
            id: "unknown.potion",
        }),
        RuleCollectionLookup::Unsupported {
            family: RuleFamily::PotionInteraction,
            ..
        }
    ));
    assert!(matches!(
        lookup_rules(
            RuleQuery::by_id(RuleId::SyntheticModifierOrdering).in_context(
                sts2_game_core::RuleContext {
                    content: "wrong",
                    build: "synthetic-build-v1",
                    mode: GameMode::Solo,
                }
            )
        ),
        RuleCollectionLookup::Unsupported { .. }
    ));
}

#[test]
fn compound_queries_never_return_a_partial_success() {
    let query = RuleQuery {
        rule_id: Some(RuleId::NominalCardDamage),
        mechanic: Some(RuleFamily::Damage),
        entity: None,
        context: None,
    };
    assert!(matches!(
        lookup_rules(query),
        RuleCollectionLookup::Unsupported { .. }
    ));
    assert!(coverage_inventory().contains(&RuleFamily::CoopScaling));
    assert!(
        coverage_for(RuleFamily::Damage)
            .unmodeled
            .contains("mitigation")
    );
    let damage = lookup_rules(RuleQuery::by_mechanic(RuleFamily::Damage));
    assert!(matches!(damage, RuleCollectionLookup::Conditional { .. }));
    let RuleCollectionLookup::Conditional { matches, .. } = damage else {
        return;
    };
    assert!(matches.as_slice().iter().all(|rule| {
        rule.version == sts2_game_core::RULES_REFERENCE_VERSION
            && rule.evidence != EvidenceStatus::Confirmed
    }));
}
