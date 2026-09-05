// SPDX-License-Identifier: MIT

use sts2_game_core::{
    BeliefOutcome, BeliefState, CalculatorError, CardSpec, CardTarget, CombatCalculationState,
    EnemyFacts, TargetDomain, exact_card_damage, exact_end_turn_survival, exact_lethal,
    exact_resource_after_card, simulate_end_turn,
};

fn state() -> CombatCalculationState {
    CombatCalculationState {
        player_hp: 65535,
        max_player_hp: 65535,
        player_block: 0,
        incoming_damage: 0,
        energy: 255,
        enemies: vec![],
    }
}

#[test]
fn maximum_width_damage_is_exact_without_wrapping() {
    let mut facts = state();
    facts.enemies = (1..=256)
        .map(|enemy_id| EnemyFacts {
            enemy_id,
            hp: 65535,
            max_hp: 65535,
        })
        .collect();
    let card = CardSpec {
        card_id: 1,
        cost: 255,
        damage: 65535,
        hits: 255,
        block: 65535,
        target: TargetDomain::AllEnemies,
    };
    assert_eq!(
        exact_card_damage(&card, &facts, CardTarget::AllEnemies)
            .unwrap()
            .damage,
        4_278_124_800
    );
    assert!(exact_lethal(&card, &facts, CardTarget::AllEnemies).unwrap());
    assert_eq!(
        exact_resource_after_card(&card, &facts)
            .unwrap()
            .energy_after,
        0
    );
    facts.enemies.push(EnemyFacts {
        enemy_id: 257,
        hp: 1,
        max_hp: 1,
    });
    assert_eq!(facts.validate(), Err(CalculatorError::MalformedObservation));
}

#[test]
fn survival_matches_integer_balance_for_all_small_inputs() {
    for hp in 0..=12 {
        for block in 0..=12 {
            for incoming_damage in 0..=12 {
                let facts = CombatCalculationState {
                    player_hp: hp,
                    player_block: block,
                    incoming_damage,
                    ..state()
                };
                let result = exact_end_turn_survival(&facts).unwrap();
                // Independent signed balance oracle; no saturating calculator operations.
                let unblocked = (i32::from(incoming_damage) - i32::from(block)).max(0);
                let remaining = (i32::from(hp) - unblocked).max(0);
                assert_eq!(i32::from(result.damage_to_player), unblocked);
                assert_eq!(i32::from(result.hp_after), remaining);
                assert_eq!(result.survives, remaining > 0);
            }
        }
    }
}

#[test]
fn belief_validation_and_maximum_weights_are_bounded() {
    assert!(BeliefState::new(vec![]).is_err());
    assert!(
        BeliefState::new(vec![BeliefOutcome {
            state: state(),
            weight: 0
        }])
        .is_err()
    );
    let outcome = BeliefOutcome {
        state: state(),
        weight: u32::MAX,
    };
    assert!(BeliefState::new(vec![outcome.clone(); 257]).is_err());
    let belief = BeliefState::new(vec![outcome; 256]).unwrap();
    let summary = simulate_end_turn(&belief).unwrap();
    assert_eq!(summary.total_weight, 1_099_511_627_520);
    assert_eq!(summary.surviving_weight, summary.total_weight);
    assert_eq!(summary.hp_after_weighted_sum, 72_056_494_509_523_200);
    assert_eq!(
        belief.survival_estimate().unwrap().numerator(),
        summary.total_weight
    );
}

#[test]
fn area_lethal_means_at_least_one_target_not_whole_encounter() {
    let mut facts = state();
    facts.enemies = vec![
        EnemyFacts {
            enemy_id: 1,
            hp: 5,
            max_hp: 5,
        },
        EnemyFacts {
            enemy_id: 2,
            hp: 100,
            max_hp: 100,
        },
    ];
    let card = CardSpec {
        card_id: 1,
        cost: 0,
        damage: 5,
        hits: 1,
        block: 0,
        target: TargetDomain::AllEnemies,
    };
    assert!(exact_lethal(&card, &facts, CardTarget::AllEnemies).unwrap());
    assert_eq!(
        exact_card_damage(&card, &facts, CardTarget::AllEnemies)
            .unwrap()
            .damage,
        10
    );
    facts.enemies.clear();
    assert!(!exact_lethal(&card, &facts, CardTarget::AllEnemies).unwrap());
}
