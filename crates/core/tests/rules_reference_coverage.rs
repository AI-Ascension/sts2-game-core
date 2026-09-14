// SPDX-License-Identifier: MIT

//! Coverage tests for the broadened rules-reference inventory.

use sts2_game_core::{
    ASCENSION_ENEMY_HP_FIXTURE, BLOCK_EXPIRY_CLAMPED_FIXTURE, BLOCK_EXPIRY_FIXTURE,
    CARD_MULTI_HIT_ORDERING_FIXTURE, COOP_ENEMY_HP_FIXTURE, CardSpec, CardTarget,
    CombatCalculationState, DRAW_TO_HAND_SIZE_FIXTURE, DRAW_WITH_FULL_HAND_FIXTURE,
    ENERGY_CAP_FIXTURE, EXHAUST_ZONE_MOVE_FIXTURE, EnemyFacts, EntityKind, EntityReference,
    EvidenceStatus, FixtureClass, FixtureOp, MAX_RULE_MATCHES, OrderedFixture,
    POTION_CONSUMPTION_FIXTURE, RELIC_FLAT_BONUS_ORDERING_FIXTURE, REWARD_CHOICE_FIXTURE,
    REWARD_CHOICE_OUT_OF_COVERAGE_FIXTURE, RuleCollectionLookup, RuleCoverageStatus, RuleFamily,
    RuleId, RuleLookup, RuleMatches, RuleQuery, RuleReference, RuleSupport,
    SATURATING_HEAL_FIXTURE, SIMPLIFIED_NOMINAL_DAMAGE_FIXTURE, START_OF_TURN_ENERGY_FIXTURE,
    STATUS_VULNERABLE_FIXTURE, SourceStatus, TargetDomain, coverage_for, coverage_inventory,
    exact_card_damage, lookup_rules, rule_inventory, rules_for, rules_for_entity,
    rules_for_mechanic, unmodeled_combinations, unmodeled_for,
};

const FLOOR_BY_ZERO: &[FixtureOp] = &[FixtureOp::FloorDivide(0)];
const CEIL_BY_ZERO: &[FixtureOp] = &[FixtureOp::CeilDivide(0)];
const HALF_UP_BY_ZERO: &[FixtureOp] = &[FixtureOp::HalfUpDivide(0)];

fn catalog_family(family: RuleFamily) -> Vec<RuleReference> {
    rule_inventory()
        .iter()
        .copied()
        .filter(|rule| rule.family == family)
        .collect()
}

/// Every entity reference named by at least one record, derived from record metadata only.
fn record_entities() -> Vec<EntityReference<'static>> {
    let mut entities = Vec::new();
    for rule in rule_inventory() {
        for entity in rule.entities {
            if !entities.contains(entity) {
                entities.push(*entity);
            }
        }
    }
    entities
}

/// Returns the bounded matches of a lookup, asserting the variant record metadata requires.
///
/// A non-empty set is `Found` only when every expected record is `Supported` and its family
/// declares no unmodeled combination, and an empty set must be `Unsupported`.
fn required_matches(
    lookup: RuleCollectionLookup,
    expected: &[RuleReference],
    label: &str,
) -> RuleMatches {
    let conditional = expected.iter().any(|rule| {
        rule.support != RuleSupport::Supported || !unmodeled_combinations(rule.family).is_empty()
    });
    assert_eq!(
        (lookup.is_conditional(), lookup.matches().is_some()),
        (conditional, !expected.is_empty()),
        "{label} result variant"
    );
    lookup.matches().unwrap_or(RuleMatches { rules: &[] })
}

fn fixture(
    rule: RuleId,
    start: u64,
    ops: &'static [FixtureOp],
    expected: Option<u64>,
) -> OrderedFixture {
    OrderedFixture {
        rule,
        class: FixtureClass::ConfirmedSyntheticRule,
        start,
        ops,
        expected,
    }
}

#[test]
fn coverage_rows_match_the_bounded_catalog() {
    assert_eq!(coverage_inventory().len(), 14);
    assert_eq!(rule_inventory().len(), 16);
    for family in coverage_inventory() {
        let row = coverage_for(*family);
        let expected = catalog_family(*family);
        assert_eq!(row.rules.len(), expected.len(), "family {family:?}");
        assert!(
            row.rules
                .iter()
                .zip(expected.iter())
                .all(|(left, right)| left.id == right.id),
            "family {family:?} index disagrees with record metadata"
        );
        assert!(
            !row.unmodeled.is_empty(),
            "family {family:?} has no exclusions"
        );
        let status = if expected.is_empty() {
            RuleCoverageStatus::Unmodeled
        } else {
            RuleCoverageStatus::Partial
        };
        assert_eq!(row.status, status, "family {family:?}");
        let lookup = rules_for_mechanic(*family);
        let matches = required_matches(lookup, &expected, &format!("family {family:?}"));
        assert_eq!(matches.len(), expected.len(), "family {family:?}");
        assert!(matches.len() <= MAX_RULE_MATCHES);
    }
    let ids: Vec<RuleId> = rule_inventory().iter().map(|rule| rule.id).collect();
    for (position, id) in ids.iter().enumerate() {
        assert!(!ids[..position].contains(id), "duplicate record {id:?}");
    }
}

#[test]
fn entity_lookup_is_bounded_and_matches_record_metadata() {
    for rule in rule_inventory() {
        let by_rule = rules_for_entity(EntityReference {
            kind: EntityKind::Rule,
            id: rule.id.as_str(),
        });
        let expected = std::slice::from_ref(rule);
        let matches = required_matches(by_rule, expected, &format!("rule {:?}", rule.id));
        assert_eq!(matches.len(), 1, "rule reference {:?}", rule.id);
        assert_eq!(matches.as_slice()[0].id, rule.id);
    }
    let entities = record_entities();
    assert!(!entities.is_empty(), "no record names an entity reference");
    for entity in entities {
        let expected: Vec<RuleReference> = rule_inventory()
            .iter()
            .copied()
            .filter(|rule| rule.entities.contains(&entity))
            .collect();
        assert!(!expected.is_empty(), "entity {entity:?} names no record");
        let label = format!("entity {entity:?}");
        let by_entity = required_matches(rules_for_entity(entity), &expected, &label);
        let mut found: Vec<&str> = by_entity
            .rules
            .iter()
            .map(|rule| rule.id.as_str())
            .collect();
        let mut ids: Vec<&str> = expected.iter().map(|rule| rule.id.as_str()).collect();
        ids.sort_unstable();
        found.sort_unstable();
        assert_eq!(
            found, ids,
            "entity {entity:?} index disagrees with record metadata"
        );
        assert!(by_entity.len() <= MAX_RULE_MATCHES);
    }
    assert!(matches!(
        rules_for_entity(EntityReference {
            kind: EntityKind::Relic,
            id: "unknown.relic",
        }),
        RuleCollectionLookup::Unsupported {
            family: RuleFamily::RelicInteraction,
            ..
        }
    ));
    assert!(matches!(
        rules_for_entity(EntityReference {
            kind: EntityKind::Rule,
            id: "no.such_rule",
        }),
        RuleCollectionLookup::Unsupported {
            family: RuleFamily::EntityInteraction,
            ..
        }
    ));
}

#[test]
fn confirmed_synthetic_rules_are_distinguished_from_simplified_estimates() {
    every_record_declares_a_guarded_evidence_class();
}

#[test]
fn every_record_declares_a_guarded_evidence_class() {
    let mut counts = [0usize; 3];
    for rule in rule_inventory() {
        assert!(!rule.claims_native_source(), "{:?}", rule.id);
        assert!(!rule.unmodeled.is_empty(), "{:?}", rule.id);
        match rule.evidence {
            EvidenceStatus::Confirmed => {
                counts[0] += 1;
                assert_eq!(rule.source, SourceStatus::Synthetic, "{:?}", rule.id);
                assert_eq!(rule.support, RuleSupport::Supported, "{:?}", rule.id);
                assert!(
                    rule.assumptions.contains("not host parity evidence"),
                    "{:?}",
                    rule.id
                );
            }
            EvidenceStatus::SimplifiedModel => {
                counts[1] += 1;
                assert_eq!(rule.source, SourceStatus::Synthetic, "{:?}", rule.id);
                assert_eq!(rule.support, RuleSupport::Conditional, "{:?}", rule.id);
                assert!(!rule.assumptions.is_empty(), "{:?}", rule.id);
            }
            EvidenceStatus::SyntheticFixture => {
                counts[2] += 1;
                assert_eq!(rule.source, SourceStatus::Synthetic, "{:?}", rule.id);
                assert_eq!(rule.support, RuleSupport::Conditional, "{:?}", rule.id);
                assert!(!rule.assumptions.is_empty(), "{:?}", rule.id);
            }
            other => panic!("unguarded evidence class {other:?} for {:?}", rule.id),
        }
    }
    assert_eq!(counts, [11, 3, 2]);
    assert_eq!(counts.iter().sum::<usize>(), rule_inventory().len());
}

#[test]
fn interacting_modifiers_verify_declared_order() {
    assert_eq!(
        CARD_MULTI_HIT_ORDERING_FIXTURE.evaluate(),
        Some((5 + 2) * 3 / 2 * 2)
    );
    assert_eq!(
        RELIC_FLAT_BONUS_ORDERING_FIXTURE.evaluate(),
        Some(5 * 3 / 2 + 2)
    );
    assert_ne!(
        CARD_MULTI_HIT_ORDERING_FIXTURE.evaluate(),
        RELIC_FLAT_BONUS_ORDERING_FIXTURE.evaluate()
    );
    assert!(CARD_MULTI_HIT_ORDERING_FIXTURE.matches_expected());
    assert!(RELIC_FLAT_BONUS_ORDERING_FIXTURE.matches_expected());
    let reordered = fixture(
        RuleId::CardMultiHitOrdering,
        5,
        &[
            FixtureOp::Multiply(3),
            FixtureOp::FloorDivide(2),
            FixtureOp::Add(2),
            FixtureOp::Multiply(2),
        ],
        Some(18),
    );
    assert_eq!(reordered.evaluate(), Some(18));
    assert_ne!(
        reordered.evaluate(),
        CARD_MULTI_HIT_ORDERING_FIXTURE.evaluate()
    );
}

#[test]
fn declared_rounding_policies_change_the_settled_value() {
    let base: u64 = 41;
    let ceil_case = fixture(
        RuleId::AscensionEnemyHpScaling,
        base,
        &[FixtureOp::Multiply(12), FixtureOp::CeilDivide(10)],
        Some(50),
    );
    let floor_case = fixture(
        RuleId::AscensionEnemyHpScaling,
        base,
        &[FixtureOp::Multiply(12), FixtureOp::FloorDivide(10)],
        Some(49),
    );
    assert_eq!(ceil_case.evaluate(), Some((base * 12).div_ceil(10)));
    assert_eq!(floor_case.evaluate(), Some(base * 12 / 10));
    assert_ne!(ceil_case.evaluate(), floor_case.evaluate());
    let half_up = fixture(
        RuleId::CoopEnemyHpScaling,
        10,
        &[FixtureOp::HalfUpDivide(4)],
        Some(3),
    );
    let floored = fixture(
        RuleId::CoopEnemyHpScaling,
        10,
        &[FixtureOp::FloorDivide(4)],
        Some(2),
    );
    assert_eq!(half_up.evaluate(), Some(3));
    assert_eq!(floored.evaluate(), Some(2));
    assert_ne!(half_up.evaluate(), floored.evaluate());
    for asserted in [
        SATURATING_HEAL_FIXTURE,
        DRAW_TO_HAND_SIZE_FIXTURE,
        DRAW_WITH_FULL_HAND_FIXTURE,
        EXHAUST_ZONE_MOVE_FIXTURE,
        START_OF_TURN_ENERGY_FIXTURE,
        ENERGY_CAP_FIXTURE,
        BLOCK_EXPIRY_FIXTURE,
        BLOCK_EXPIRY_CLAMPED_FIXTURE,
        POTION_CONSUMPTION_FIXTURE,
        ASCENSION_ENEMY_HP_FIXTURE,
        COOP_ENEMY_HP_FIXTURE,
        STATUS_VULNERABLE_FIXTURE,
    ] {
        assert!(asserted.matches_expected(), "{:?}", asserted.rule);
    }
    assert_eq!(
        SATURATING_HEAL_FIXTURE.evaluate(),
        Some(8_u64.saturating_add(5).min(9))
    );
    assert_eq!(DRAW_WITH_FULL_HAND_FIXTURE.evaluate(), Some(0));
    assert_eq!(ENERGY_CAP_FIXTURE.evaluate(), Some(3));
    assert_eq!(BLOCK_EXPIRY_FIXTURE.evaluate(), Some(5 - 3));
    assert_eq!(BLOCK_EXPIRY_CLAMPED_FIXTURE.evaluate(), Some(0));
    assert_eq!(STATUS_VULNERABLE_FIXTURE.evaluate(), Some(7 * 3 / 2));
}

#[test]
fn out_of_coverage_requests_never_produce_a_number() {
    assert_eq!(REWARD_CHOICE_OUT_OF_COVERAGE_FIXTURE.evaluate(), None);
    assert!(REWARD_CHOICE_OUT_OF_COVERAGE_FIXTURE.matches_expected());
    assert!(REWARD_CHOICE_FIXTURE.matches_expected());
    assert_eq!(REWARD_CHOICE_FIXTURE.evaluate(), Some(1));
    for ops in [FLOOR_BY_ZERO, CEIL_BY_ZERO, HALF_UP_BY_ZERO] {
        let undefined = fixture(RuleId::RewardChoicePicks, 7, ops, None);
        assert_eq!(undefined.evaluate(), None);
        assert!(undefined.matches_expected());
    }
    let compound = RuleQuery {
        rule_id: Some(RuleId::NominalCardDamage),
        mechanic: Some(RuleFamily::Damage),
        entity: None,
        context: None,
    };
    assert!(matches!(
        lookup_rules(compound),
        RuleCollectionLookup::Unsupported { .. }
    ));
    assert!(matches!(
        rules_for_mechanic(RuleFamily::EntityInteraction),
        RuleCollectionLookup::Unsupported { .. }
    ));
    let lookup = lookup_rules(RuleQuery::by_id(RuleId::RewardChoicePicks));
    let expected = coverage_for(RuleFamily::Acquisition).rules;
    let matches = required_matches(lookup, expected, "reward choice picks");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches.as_slice()[0].id, RuleId::RewardChoicePicks);
    assert_eq!(
        lookup.reason(),
        "Only the bounded pick count is declared; the offered set, rarity weighting, and card identity are unmodeled and must not be inferred."
    );
    for family in coverage_inventory() {
        let exclusions = unmodeled_combinations(*family);
        assert!(!exclusions.is_empty(), "family {family:?}");
        for entry in exclusions {
            assert!(!entry.combination.is_empty() && !entry.reason.is_empty());
        }
    }
    assert!(unmodeled_combinations(RuleFamily::Damage).len() >= 2);
    for rule in rule_inventory() {
        assert!(
            unmodeled_for(rule.id).is_some_and(|entries| !entries.is_empty()),
            "{:?}",
            rule.id
        );
    }
}

#[test]
fn simplified_estimates_stay_separate_from_confirmed_rules() {
    assert_eq!(
        SIMPLIFIED_NOMINAL_DAMAGE_FIXTURE.nominal_damage(),
        Some(SIMPLIFIED_NOMINAL_DAMAGE_FIXTURE.expected)
    );
    assert_ne!(
        SIMPLIFIED_NOMINAL_DAMAGE_FIXTURE
            .nominal_damage()
            .map(u64::from),
        CARD_MULTI_HIT_ORDERING_FIXTURE.evaluate()
    );
    let card = CardSpec {
        card_id: 1,
        cost: 0,
        damage: 5,
        hits: 2,
        block: 0,
        target: TargetDomain::SingleEnemy,
    };
    let state = CombatCalculationState {
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
    let simplified =
        exact_card_damage(&card, &state, CardTarget::Enemy(1)).map(|damage| damage.damage);
    assert_eq!(simplified, Ok(10));
    let RuleLookup::Found(nominal) = rules_for(RuleId::NominalCardDamage) else {
        return;
    };
    let RuleLookup::Found(confirmed) = rules_for(RuleId::CardMultiHitOrdering) else {
        return;
    };
    assert_eq!(nominal.evidence, EvidenceStatus::SimplifiedModel);
    assert_eq!(nominal.support, RuleSupport::Conditional);
    assert_eq!(confirmed.evidence, EvidenceStatus::Confirmed);
    assert_eq!(confirmed.support, RuleSupport::Supported);
    assert_ne!(nominal.evidence, confirmed.evidence);
    assert!(matches!(
        rules_for_mechanic(RuleFamily::Damage),
        RuleCollectionLookup::Conditional { .. }
    ));
    assert!(matches!(
        rules_for_mechanic(RuleFamily::Heal),
        RuleCollectionLookup::Conditional { .. }
    ));
}
