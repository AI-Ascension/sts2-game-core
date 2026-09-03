// SPDX-License-Identifier: MIT

use sts2_game_core::{
    CombatPhase, CombatSnapshot, Generation, Identity, PLAY_CARD_MAX_INDEX, PlayCardRequest,
    PlayCardValidationError, SessionId, TurnIndex, validate_play_card,
};

fn snapshot(phase: CombatPhase, generation: u64) -> Result<CombatSnapshot, &'static str> {
    let actor = Identity::new(7).ok_or("non-zero actor")?;
    let session = SessionId::new(9).ok_or("non-zero session")?;
    Ok(CombatSnapshot::new(
        actor,
        session,
        Generation::new(generation),
        phase,
        TurnIndex::new(2),
    ))
}

fn request(generation: u64, index: u16) -> Result<PlayCardRequest, &'static str> {
    let actor = Identity::new(7).ok_or("non-zero actor")?;
    let session = SessionId::new(9).ok_or("non-zero session")?;
    Ok(PlayCardRequest::new(
        actor,
        session,
        Generation::new(generation),
        index,
        None,
    ))
}

#[test]
fn card_play_checks_identity_freshness_phase_and_hand_index() -> Result<(), String> {
    let current = snapshot(CombatPhase::PlayerTurn, 4).map_err(String::from)?;
    let valid = request(4, 0).map_err(String::from)?;
    assert!(validate_play_card(&current, &valid, 5).is_ok());
    assert_eq!(
        validate_play_card(&current, &request(3, 0).map_err(String::from)?, 5),
        Err(PlayCardValidationError::StaleGeneration)
    );
    assert_eq!(
        validate_play_card(&current, &request(4, 5).map_err(String::from)?, 5),
        Err(PlayCardValidationError::CardNotInHand)
    );
    assert_eq!(
        validate_play_card(
            &current,
            &request(4, PLAY_CARD_MAX_INDEX + 1).map_err(String::from)?,
            65,
        ),
        Err(PlayCardValidationError::CardIndexOutOfRange)
    );
    Ok(())
}

#[test]
fn card_play_rejects_non_player_turns_before_host_rules() -> Result<(), String> {
    let request = request(4, 0).map_err(String::from)?;
    assert_eq!(
        validate_play_card(
            &snapshot(CombatPhase::OutsideCombat, 4).map_err(String::from)?,
            &request,
            5,
        ),
        Err(PlayCardValidationError::OutsideCombat)
    );
    assert_eq!(
        validate_play_card(
            &snapshot(CombatPhase::EnemyTurn, 4).map_err(String::from)?,
            &request,
            5,
        ),
        Err(PlayCardValidationError::EnemyTurn)
    );
    Ok(())
}
