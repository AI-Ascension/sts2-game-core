// SPDX-License-Identifier: MIT

use sts2_game_core::{
    CardSpec, CardTarget, CombatCalculationState, CombatPhase, CombatSnapshot, EnemyFacts,
    Generation, Identity, PlayCardRequest, PlayCardValidationError, SessionId, TargetDomain,
    TurnIndex, calculate_play_card, validate_play_card,
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

fn snapshot(generation: u64) -> CombatSnapshot {
    CombatSnapshot::new(
        Identity::new(1).unwrap(),
        SessionId::initial(),
        Generation::new(generation),
        CombatPhase::PlayerTurn,
        TurnIndex::new(1),
    )
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
    let validated = validate_play_card(&state(), &snapshot(3), request, &[card()]).unwrap();
    let facts = calculate_play_card(&validated, &state(), &snapshot(3), &[card()]).unwrap();
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
    assert!(validate_play_card(&state(), &snapshot(3), request, &[card()]).is_err());
    assert!(validate_play_card(&state(), &snapshot(2), request, &[]).is_err());
}

fn request() -> PlayCardRequest {
    PlayCardRequest {
        actor: Identity::new(1).unwrap(),
        session: SessionId::initial(),
        generation: Generation::new(3),
        card_id: 4,
        target: CardTarget::Enemy(8),
    }
}

#[test]
fn rejects_foreign_scope_and_non_player_phases_in_order() {
    let cases = [
        (
            CombatSnapshot::new(
                Identity::new(2).unwrap(),
                SessionId::new(2).unwrap(),
                Generation::new(9),
                CombatPhase::EnemyTurn,
                TurnIndex::new(1),
            ),
            PlayCardValidationError::ActorMismatch,
        ),
        (
            CombatSnapshot::new(
                Identity::new(1).unwrap(),
                SessionId::new(2).unwrap(),
                Generation::new(9),
                CombatPhase::EnemyTurn,
                TurnIndex::new(1),
            ),
            PlayCardValidationError::SessionMismatch,
        ),
        (snapshot(4), PlayCardValidationError::StaleGeneration),
        (
            CombatSnapshot::new(
                Identity::new(1).unwrap(),
                SessionId::initial(),
                Generation::new(3),
                CombatPhase::OutsideCombat,
                TurnIndex::new(1),
            ),
            PlayCardValidationError::OutsideCombat,
        ),
        (
            CombatSnapshot::new(
                Identity::new(1).unwrap(),
                SessionId::initial(),
                Generation::new(3),
                CombatPhase::EnemyTurn,
                TurnIndex::new(1),
            ),
            PlayCardValidationError::EnemyTurn,
        ),
    ];
    let validated = validate_play_card(&state(), &snapshot(3), request(), &[card()]).unwrap();
    for (scope, error) in cases {
        assert_eq!(
            validate_play_card(&state(), &scope, request(), &[card()]),
            Err(error)
        );
        assert_eq!(
            calculate_play_card(&validated, &state(), &scope, &[card()]),
            Err(error)
        );
    }
}

#[test]
fn calculation_rechecks_resources_and_current_hand() {
    let validated = validate_play_card(&state(), &snapshot(3), request(), &[card()]).unwrap();
    let mut spent = state();
    spent.energy = 0;
    assert_eq!(
        calculate_play_card(&validated, &spent, &snapshot(3), &[card()]),
        Err(PlayCardValidationError::InsufficientEnergy)
    );
    assert_eq!(
        calculate_play_card(&validated, &state(), &snapshot(3), &[]),
        Err(PlayCardValidationError::CardNotInHand)
    );
    let changed = CardSpec {
        damage: 99,
        ..card()
    };
    assert_eq!(
        calculate_play_card(&validated, &state(), &snapshot(3), &[changed]),
        Err(PlayCardValidationError::InvalidCard)
    );
}

#[test]
fn validated_token_is_bound_to_all_observed_facts_not_only_generation() {
    let original = state();
    let validated = validate_play_card(&original, &snapshot(3), request(), &[card()]).unwrap();
    let mut changed = original.clone();
    changed.enemies[0].hp -= 1;
    assert_eq!(
        calculate_play_card(&validated, &changed, &snapshot(3), &[card()]),
        Err(PlayCardValidationError::StaleObservation)
    );
    let other = CardSpec {
        card_id: 5,
        ..card()
    };
    assert_eq!(
        calculate_play_card(&validated, &original, &snapshot(3), &[card(), other]),
        Err(PlayCardValidationError::StaleObservation)
    );
    assert!(calculate_play_card(&validated, &original, &snapshot(3), &[card()]).is_ok());
}

#[test]
fn duplicate_invalid_and_oversized_hands_are_not_order_dependent() {
    let duplicate = CardSpec {
        cost: 255,
        ..card()
    };
    for hand in [
        vec![card(), duplicate],
        vec![duplicate, card()],
        vec![card(); 257],
    ] {
        assert_eq!(
            validate_play_card(&state(), &snapshot(3), request(), &hand),
            Err(PlayCardValidationError::MalformedObservation)
        );
    }
    let invalid = CardSpec {
        card_id: 0,
        ..card()
    };
    assert_eq!(
        validate_play_card(&state(), &snapshot(3), request(), &[card(), invalid]),
        Err(PlayCardValidationError::InvalidCard)
    );
}
