// SPDX-License-Identifier: MIT

use sts2_game_core::{
    CardSpec, CardTarget, CombatCalculationState, EnemyFacts, TargetDomain, exact_card_damage,
    exact_end_turn_survival, exact_lethal, exact_resource_after_card,
};

fn state() -> CombatCalculationState {
    CombatCalculationState {
        player_hp: 10,
        max_player_hp: 10,
        player_block: 3,
        incoming_damage: 7,
        energy: 2,
        enemies: vec![EnemyFacts {
            enemy_id: 1,
            hp: 6,
            max_hp: 6,
        }],
    }
}

fn strike() -> CardSpec {
    CardSpec {
        card_id: 9,
        cost: 1,
        damage: 3,
        hits: 2,
        block: 0,
        target: TargetDomain::SingleEnemy,
    }
}

#[test]
fn exact_calculators_use_only_visible_facts() {
    let facts = state();
    assert_eq!(
        exact_card_damage(&strike(), &facts, CardTarget::Enemy(1)).unwrap(),
        sts2_game_core::ExactDamage {
            damage: 6,
            target_id: Some(1),
        }
    );
    assert!(exact_lethal(&strike(), &facts, CardTarget::Enemy(1)).unwrap());
    assert_eq!(
        exact_resource_after_card(&strike(), &facts).unwrap().energy_after,
        1
    );
    assert_eq!(exact_end_turn_survival(&facts).unwrap().hp_after, 6);
}

#[test]
fn calculators_reject_target_domain_and_duplicate_observation_facts() {
    let mut facts = state();
    assert!(exact_card_damage(&strike(), &facts, CardTarget::None).is_err());
    facts.enemies.push(EnemyFacts {
        enemy_id: 1,
        hp: 4,
        max_hp: 4,
    });
    assert!(exact_end_turn_survival(&facts).is_err());
}
