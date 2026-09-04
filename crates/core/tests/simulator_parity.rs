// SPDX-License-Identifier: MIT

use sts2_game_core::{
    BeliefOutcome, BeliefState, CombatCalculationState, EnemyFacts, EstimateSource,
    simulate_end_turn,
};

fn branch(incoming_damage: u16) -> CombatCalculationState {
    CombatCalculationState {
        player_hp: 10,
        max_player_hp: 10,
        player_block: 0,
        incoming_damage,
        energy: 0,
        enemies: vec![EnemyFacts {
            enemy_id: 1,
            hp: 10,
            max_hp: 10,
        }],
    }
}

#[test]
fn simulator_enumerates_only_explicit_belief_branches() {
    let belief = BeliefState::new(vec![
        BeliefOutcome {
            state: branch(2),
            weight: 1,
        },
        BeliefOutcome {
            state: branch(11),
            weight: 3,
        },
    ])
    .unwrap();
    let estimate = belief.survival_estimate().unwrap();
    let summary = simulate_end_turn(&belief).unwrap();
    assert_eq!(estimate.source, EstimateSource::ExplicitBelief);
    assert_eq!(estimate.numerator(), 1);
    assert_eq!(estimate.denominator(), 4);
    assert_eq!(summary.surviving_weight, 1);
    assert_eq!(summary.hp_after_weighted_sum, 8);
}
