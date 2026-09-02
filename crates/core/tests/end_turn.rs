// SPDX-License-Identifier: MIT

use sts2_game_core::{
    CombatPhase, CombatSnapshot, EndTurnAction, EndTurnApplyError, EndTurnRequest,
    EndTurnValidationError, Generation, Identity, SessionId, TurnIndex, validate_end_turn,
};

fn actor(value: u64) -> Result<Identity, &'static str> {
    Identity::new(value).ok_or("test actor must be non-zero")
}

fn session(value: u64) -> Result<SessionId, &'static str> {
    SessionId::new(value).ok_or("test session must be non-zero")
}

fn player_turn_snapshot() -> Result<CombatSnapshot, &'static str> {
    Ok(CombatSnapshot::new(
        actor(7)?,
        session(11)?,
        Generation::new(4),
        CombatPhase::PlayerTurn,
        TurnIndex::new(2),
    ))
}

fn end_turn_request(generation: u64) -> Result<EndTurnRequest, &'static str> {
    Ok(EndTurnRequest::new(
        actor(7)?,
        session(11)?,
        Generation::new(generation),
    ))
}

#[test]
fn valid_end_turn_settles_once_with_typed_witness() -> Result<(), &'static str> {
    let snapshot = player_turn_snapshot()?;
    let request = end_turn_request(4)?;
    let validated = validate_end_turn(&snapshot, &request)
        .map_err(|_| "valid end-turn request was rejected")?;

    assert_eq!(validated.action(), EndTurnAction::new());
    assert_eq!(validated.actor(), actor(7)?);
    assert_eq!(validated.session(), session(11)?);
    assert_eq!(validated.generation(), Generation::new(4));

    let settled = snapshot
        .apply_end_turn(&validated)
        .map_err(|_| "valid end-turn request did not settle")?;
    let next = settled.snapshot();
    let effect = settled.effect();

    assert_eq!(next.actor(), actor(7)?);
    assert_eq!(next.session(), session(11)?);
    assert_eq!(next.generation(), Generation::new(5));
    assert_eq!(next.phase(), CombatPhase::PlayerTurn);
    assert_eq!(next.turn_index(), TurnIndex::new(3));
    assert_eq!(effect.action(), EndTurnAction::new());
    assert_eq!(effect.before_generation(), Generation::new(4));
    assert_eq!(effect.after_generation(), Generation::new(5));
    assert_eq!(effect.before_turn(), TurnIndex::new(2));
    assert_eq!(effect.after_turn(), TurnIndex::new(3));

    assert_eq!(snapshot, player_turn_snapshot()?);
    assert_eq!(
        next.apply_end_turn(&validated),
        Err(EndTurnApplyError::StaleGeneration)
    );
    Ok(())
}

#[test]
fn identity_and_generation_rejections_precede_state_change() -> Result<(), &'static str> {
    let snapshot = player_turn_snapshot()?;
    let wrong_actor = EndTurnRequest::new(actor(8)?, session(11)?, Generation::new(99));
    assert_eq!(
        validate_end_turn(&snapshot, &wrong_actor),
        Err(EndTurnValidationError::ActorMismatch {
            expected: actor(7)?,
            actual: actor(8)?,
        })
    );

    let wrong_session = EndTurnRequest::new(actor(7)?, session(12)?, Generation::new(99));
    assert_eq!(
        validate_end_turn(&snapshot, &wrong_session),
        Err(EndTurnValidationError::SessionMismatch {
            expected: session(11)?,
            actual: session(12)?,
        })
    );

    let stale = end_turn_request(3)?;
    assert_eq!(
        validate_end_turn(&snapshot, &stale),
        Err(EndTurnValidationError::StaleGeneration {
            expected: Generation::new(3),
            actual: Generation::new(4),
        })
    );
    assert_eq!(snapshot, player_turn_snapshot()?);
    Ok(())
}

#[test]
fn phase_and_bounds_rejections_precede_state_change() -> Result<(), &'static str> {
    let request = end_turn_request(4)?;
    let outside = CombatSnapshot::new(
        actor(7)?,
        session(11)?,
        Generation::new(4),
        CombatPhase::OutsideCombat,
        TurnIndex::new(2),
    );
    assert_eq!(
        validate_end_turn(&outside, &request),
        Err(EndTurnValidationError::OutsideCombat)
    );

    let enemy_turn = CombatSnapshot::new(
        actor(7)?,
        session(11)?,
        Generation::new(4),
        CombatPhase::EnemyTurn,
        TurnIndex::new(2),
    );
    assert_eq!(
        validate_end_turn(&enemy_turn, &request),
        Err(EndTurnValidationError::EnemyTurn)
    );

    let exhausted_turn = CombatSnapshot::new(
        actor(7)?,
        session(11)?,
        Generation::new(4),
        CombatPhase::PlayerTurn,
        TurnIndex::new(u32::MAX),
    );
    assert_eq!(
        validate_end_turn(&exhausted_turn, &request),
        Err(EndTurnValidationError::TurnIndexExhausted)
    );

    let exhausted_generation = CombatSnapshot::new(
        actor(7)?,
        session(11)?,
        Generation::new(u64::MAX),
        CombatPhase::PlayerTurn,
        TurnIndex::new(2),
    );
    let max_generation_request =
        EndTurnRequest::new(actor(7)?, session(11)?, Generation::new(u64::MAX));
    assert_eq!(
        validate_end_turn(&exhausted_generation, &max_generation_request),
        Err(EndTurnValidationError::GenerationExhausted)
    );

    assert_eq!(outside.phase(), CombatPhase::OutsideCombat);
    assert_eq!(enemy_turn.phase(), CombatPhase::EnemyTurn);
    assert_eq!(exhausted_turn.turn_index(), TurnIndex::new(u32::MAX));
    assert_eq!(exhausted_generation.generation(), Generation::new(u64::MAX));
    Ok(())
}

#[test]
fn repeated_evaluation_is_stable_and_side_effect_free() -> Result<(), &'static str> {
    let snapshot = player_turn_snapshot()?;
    let request = end_turn_request(4)?;
    let expected = validate_end_turn(&snapshot, &request)
        .map_err(|_| "valid end-turn request was rejected")?;

    for _ in 0..64 {
        assert_eq!(validate_end_turn(&snapshot, &request), Ok(expected));
    }

    let first = snapshot
        .apply_end_turn(&expected)
        .map_err(|_| "first pure transition failed")?;
    let second = snapshot
        .apply_end_turn(&expected)
        .map_err(|_| "second evaluation of pure transition failed")?;
    assert_eq!(first, second);
    assert_eq!(snapshot, player_turn_snapshot()?);
    Ok(())
}
