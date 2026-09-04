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

#[test]
fn rejects_identity_and_session_before_other_invalid_fields() -> Result<(), &'static str> {
    let current = snapshot(CombatPhase::EnemyTurn, 4)?;
    let actor = Identity::new(7).ok_or("actor")?;
    let wrong_actor = Identity::new(8).ok_or("other actor")?;
    let session = SessionId::new(9).ok_or("session")?;
    let wrong_session = SessionId::new(10).ok_or("other session")?;
    for (request, expected) in [
        (
            PlayCardRequest::new(wrong_actor, wrong_session, Generation::new(3), 65, None),
            PlayCardValidationError::ActorMismatch,
        ),
        (
            PlayCardRequest::new(actor, wrong_session, Generation::new(3), 65, None),
            PlayCardValidationError::SessionMismatch,
        ),
        (
            PlayCardRequest::new(actor, session, Generation::new(3), 65, None),
            PlayCardValidationError::StaleGeneration,
        ),
        (
            PlayCardRequest::new(actor, session, Generation::new(4), 65, None),
            PlayCardValidationError::EnemyTurn,
        ),
    ] {
        assert_eq!(validate_play_card(&current, &request, 0), Err(expected));
    }
    assert_eq!(current, snapshot(CombatPhase::EnemyTurn, 4)?);
    Ok(())
}

#[test]
fn accepts_inclusive_index_limit_and_preserves_opaque_targets() -> Result<(), &'static str> {
    let current = snapshot(CombatPhase::PlayerTurn, 4)?;
    for target in [None, Identity::new(999)] {
        let proposal = PlayCardRequest::new(
            current.actor(),
            current.session(),
            current.generation(),
            PLAY_CARD_MAX_INDEX,
            target,
        );
        let accepted = validate_play_card(&current, &proposal, 65).map_err(|_| "valid proposal")?;
        assert_eq!(accepted.request(), proposal);
        assert_eq!(accepted.request().target_id(), target);
        assert_eq!(accepted.request().card_index(), 64);
        assert_eq!(accepted.request().actor(), current.actor());
        assert_eq!(accepted.request().session(), current.session());
        assert_eq!(
            accepted.request().expected_generation(),
            current.generation()
        );
        for _ in 0..16 {
            assert_eq!(validate_play_card(&current, &proposal, 65), Ok(accepted));
        }
    }
    assert_eq!(current, snapshot(CombatPhase::PlayerTurn, 4)?);
    assert_eq!(
        validate_play_card(&current, &request(4, 0)?, 0),
        Err(PlayCardValidationError::CardNotInHand)
    );
    Ok(())
}

#[test]
fn hand_checks_precede_generation_exhaustion() -> Result<(), &'static str> {
    let current = snapshot(CombatPhase::PlayerTurn, u64::MAX)?;
    assert_eq!(
        validate_play_card(&current, &request(u64::MAX, 65)?, 0),
        Err(PlayCardValidationError::CardIndexOutOfRange)
    );
    assert_eq!(
        validate_play_card(&current, &request(u64::MAX, 0)?, 0),
        Err(PlayCardValidationError::CardNotInHand)
    );
    assert_eq!(
        validate_play_card(&current, &request(u64::MAX, 0)?, 1),
        Err(PlayCardValidationError::GenerationExhausted)
    );
    assert!(
        validate_play_card(
            &snapshot(CombatPhase::PlayerTurn, u64::MAX - 1)?,
            &request(u64::MAX - 1, 0)?,
            1
        )
        .is_ok()
    );
    assert_eq!(current, snapshot(CombatPhase::PlayerTurn, u64::MAX)?);
    Ok(())
}
