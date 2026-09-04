// SPDX-License-Identifier: MIT

use sts2_game_core::{
    CardSpec, CardTarget, CombatCalculationState, EnemyFacts, Generation, Identity, PlayCardRequest,
    SessionId, TargetDomain, calculate_play_card, validate_play_card,
};

fn card() -> CardSpec {
    CardSpec {
        card_id: 4,
        cost: 1,
        damage: 5,
        hits: 1,
        block: 2,
        target: TargetDomain::SingleEnemy,
    }
}

fn state() -> CombatCalculationState {
    CombatCalculationState {
        player_hp: 20,
        max_player_hp: 20,
        player_block: 0,
        incoming_damage: 0,
        energy: 2,
        enemies: vec![EnemyFacts {
            enemy_id: 8,
            hp: 10,
            max_hp: 10,
        }],
    }
}

#[test]
fn valid_play_card_is_typed_and_pure() {
    let request = PlayCardRequest {
        actor: Identity::new(1).unwrap(),
        session: SessionId::initial(),
        generation: Generation::new(3),
        card_id: 4,
        target: CardTarget::Enemy(8),
    };
    let validated = validate_play_card(&state(), Generation::new(3), request, &[card()]).unwrap();
    let facts = calculate_play_card(validated, &state()).unwrap();
    assert_eq!(facts.damage, 5);
    assert_eq!(facts.energy_spent, 1);
}

#[test]
fn stale_or_missing_cards_are_rejected_before_calculation() {
    let request = PlayCardRequest {
        actor: Identity::new(1).unwrap(),
        session: SessionId::initial(),
        generation: Generation::new(2),
        card_id: 4,
        target: CardTarget::Enemy(8),
    };
    assert!(validate_play_card(&state(), Generation::new(3), request, &[card()]).is_err());
    assert!(validate_play_card(&state(), Generation::new(2), request, &[]).is_err());
}
