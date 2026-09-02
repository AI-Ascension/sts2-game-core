// SPDX-License-Identifier: MIT

use std::convert::TryFrom;

use sts2_game_core::{
    CombatPhase, CombatSnapshot, Generation, Identity, RUNTIME_V2_MAX_GENERATION,
    RUNTIME_V2_MAX_TURN_INDEX, RuntimeV2Generation, RuntimeV2ProjectionError, RuntimeV2TurnIndex,
    SessionId, TurnIndex,
};

fn actor(value: u64) -> Result<Identity, &'static str> {
    Identity::new(value).ok_or("test actor must be non-zero")
}

fn session(value: u64) -> Result<SessionId, &'static str> {
    SessionId::new(value).ok_or("test session must be non-zero")
}

fn snapshot(generation: u64, turn_index: u32) -> Result<CombatSnapshot, &'static str> {
    Ok(CombatSnapshot::new(
        actor(7)?,
        session(11)?,
        Generation::new(generation),
        CombatPhase::PlayerTurn,
        TurnIndex::new(turn_index),
    ))
}

#[test]
fn turn_index_projection_accepts_inclusive_limit_and_rejects_next_value() {
    assert_eq!(
        TurnIndex::new(0)
            .try_runtime_v2()
            .map(RuntimeV2TurnIndex::value),
        Ok(0)
    );
    assert_eq!(
        TurnIndex::new(RUNTIME_V2_MAX_TURN_INDEX)
            .try_runtime_v2()
            .map(RuntimeV2TurnIndex::value),
        Ok(RUNTIME_V2_MAX_TURN_INDEX)
    );
    assert_eq!(
        TurnIndex::new(RUNTIME_V2_MAX_TURN_INDEX + 1).try_runtime_v2(),
        Err(RuntimeV2ProjectionError::TurnIndexOutOfRange {
            value: RUNTIME_V2_MAX_TURN_INDEX + 1,
            max: RUNTIME_V2_MAX_TURN_INDEX,
        })
    );
}

#[test]
fn generation_projection_accepts_safe_integer_limit_and_rejects_next_value() {
    assert_eq!(
        Generation::new(0)
            .try_runtime_v2()
            .map(RuntimeV2Generation::value),
        Ok(0)
    );
    assert_eq!(
        Generation::new(RUNTIME_V2_MAX_GENERATION)
            .try_runtime_v2()
            .map(RuntimeV2Generation::value),
        Ok(RUNTIME_V2_MAX_GENERATION)
    );
    assert_eq!(
        Generation::new(RUNTIME_V2_MAX_GENERATION + 1).try_runtime_v2(),
        Err(RuntimeV2ProjectionError::GenerationOutOfRange {
            value: RUNTIME_V2_MAX_GENERATION + 1,
            max: RUNTIME_V2_MAX_GENERATION,
        })
    );
}

#[test]
fn try_from_uses_the_same_checked_projection_boundary() {
    assert_eq!(
        RuntimeV2TurnIndex::try_from(TurnIndex::new(RUNTIME_V2_MAX_TURN_INDEX))
            .map(RuntimeV2TurnIndex::value),
        Ok(RUNTIME_V2_MAX_TURN_INDEX)
    );
    assert_eq!(
        RuntimeV2Generation::try_from(Generation::new(RUNTIME_V2_MAX_GENERATION))
            .map(RuntimeV2Generation::value),
        Ok(RUNTIME_V2_MAX_GENERATION)
    );
}

#[test]
fn observation_projection_rejects_unrepresentable_values_without_changing_domain()
-> Result<(), &'static str> {
    let valid = snapshot(RUNTIME_V2_MAX_GENERATION, RUNTIME_V2_MAX_TURN_INDEX)
        .map_err(|_| "test snapshot identities are valid")?;
    let projected = valid
        .try_runtime_v2()
        .map_err(|_| "inclusive Runtime-v2 limits should project")?;
    assert_eq!(projected.generation().value(), RUNTIME_V2_MAX_GENERATION);
    assert_eq!(projected.turn_index().value(), RUNTIME_V2_MAX_TURN_INDEX);

    let invalid_generation = snapshot(RUNTIME_V2_MAX_GENERATION + 1, RUNTIME_V2_MAX_TURN_INDEX)
        .map_err(|_| "test snapshot identities are valid")?;
    assert_eq!(
        invalid_generation.try_runtime_v2(),
        Err(RuntimeV2ProjectionError::GenerationOutOfRange {
            value: RUNTIME_V2_MAX_GENERATION + 1,
            max: RUNTIME_V2_MAX_GENERATION,
        })
    );

    let invalid_turn = snapshot(RUNTIME_V2_MAX_GENERATION, RUNTIME_V2_MAX_TURN_INDEX + 1)
        .map_err(|_| "test snapshot identities are valid")?;
    assert_eq!(
        invalid_turn.try_runtime_v2(),
        Err(RuntimeV2ProjectionError::TurnIndexOutOfRange {
            value: RUNTIME_V2_MAX_TURN_INDEX + 1,
            max: RUNTIME_V2_MAX_TURN_INDEX,
        })
    );
    assert_eq!(valid.generation().value(), RUNTIME_V2_MAX_GENERATION);
    assert_eq!(valid.turn_index().value(), RUNTIME_V2_MAX_TURN_INDEX);
    Ok(())
}

#[test]
fn existing_domain_constructors_keep_their_v1_values() {
    assert_eq!(
        Generation::new(RUNTIME_V2_MAX_GENERATION + 1).value(),
        RUNTIME_V2_MAX_GENERATION + 1
    );
    assert_eq!(
        TurnIndex::new(RUNTIME_V2_MAX_TURN_INDEX + 1).value(),
        RUNTIME_V2_MAX_TURN_INDEX + 1
    );
}
