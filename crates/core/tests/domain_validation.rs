// SPDX-License-Identifier: MIT

use sts2_game_core::{
    Action, Generation, Identity, Phase, Request, State, ValidatedAction, ValidationError, validate,
};

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Accepted(Action),
    Rejected(ValidationError),
}

fn identity(value: u64) -> Result<Identity, &'static str> {
    Identity::new(value).ok_or("test identity must be non-zero")
}

fn open_state() -> Result<State, &'static str> {
    Ok(State::new(identity(7)?, Generation::new(4), Phase::Open, 3))
}

fn classify(result: Result<ValidatedAction, ValidationError>) -> Outcome {
    match result {
        Ok(accepted) => Outcome::Accepted(accepted.action()),
        Err(error) => Outcome::Rejected(error),
    }
}

#[test]
fn accepts_valid_action_without_mutating_snapshot() -> Result<(), &'static str> {
    let state = open_state()?;
    let before = state;
    let request = Request::new(
        identity(7)?,
        Generation::new(4),
        Action::UseBudget { units: 2 },
    );

    let accepted = validate(&state, &request).map_err(|_| "valid request was rejected")?;
    assert_eq!(accepted.actor(), identity(7)?);
    assert_eq!(accepted.generation(), Generation::new(4));
    assert_eq!(accepted.action(), Action::UseBudget { units: 2 });
    assert_eq!(state, before);
    Ok(())
}

#[test]
fn rejects_invalid_budget_values_deterministically() -> Result<(), &'static str> {
    let state = open_state()?;
    let cases = [
        (0, Outcome::Rejected(ValidationError::ZeroUnits)),
        (
            4,
            Outcome::Rejected(ValidationError::InsufficientUnits {
                requested: 4,
                available: 3,
            }),
        ),
    ];

    for (units, expected) in cases {
        let request = Request::new(
            identity(7)?,
            Generation::new(4),
            Action::UseBudget { units },
        );
        assert_eq!(classify(validate(&state, &request)), expected);
    }
    Ok(())
}

#[test]
fn rejects_actor_mismatch_before_other_request_checks() -> Result<(), &'static str> {
    let state = open_state()?;
    let request = Request::new(
        identity(8)?,
        Generation::new(99),
        Action::UseBudget { units: 99 },
    );

    assert_eq!(
        validate(&state, &request),
        Err(ValidationError::ActorMismatch {
            expected: identity(7)?,
            actual: identity(8)?,
        })
    );
    Ok(())
}

#[test]
fn rejects_stale_generation_before_action_rules() -> Result<(), &'static str> {
    let state = open_state()?;
    let request = Request::new(
        identity(7)?,
        Generation::new(3),
        Action::UseBudget { units: 0 },
    );

    assert_eq!(
        validate(&state, &request),
        Err(ValidationError::StaleGeneration {
            expected: Generation::new(3),
            actual: Generation::new(4),
        })
    );
    Ok(())
}

#[test]
fn rejects_actions_for_closed_state() -> Result<(), &'static str> {
    let state = State::new(identity(7)?, Generation::new(4), Phase::Closed, 3);
    let request = Request::new(
        identity(7)?,
        Generation::new(4),
        Action::UseBudget { units: 1 },
    );

    assert_eq!(
        validate(&state, &request),
        Err(ValidationError::ClosedState)
    );
    Ok(())
}

#[test]
fn golden_vectors_are_reproducible_across_repeated_evaluation() -> Result<(), &'static str> {
    let state = open_state()?;
    let vectors = [
        (
            Request::new(
                identity(7)?,
                Generation::new(4),
                Action::UseBudget { units: 1 },
            ),
            Outcome::Accepted(Action::UseBudget { units: 1 }),
        ),
        (
            Request::new(identity(7)?, Generation::new(4), Action::Close),
            Outcome::Accepted(Action::Close),
        ),
        (
            Request::new(
                identity(7)?,
                Generation::new(4),
                Action::UseBudget { units: 5 },
            ),
            Outcome::Rejected(ValidationError::InsufficientUnits {
                requested: 5,
                available: 3,
            }),
        ),
        (
            Request::new(
                identity(7)?,
                Generation::new(2),
                Action::UseBudget { units: 1 },
            ),
            Outcome::Rejected(ValidationError::StaleGeneration {
                expected: Generation::new(2),
                actual: Generation::new(4),
            }),
        ),
    ];
    let expected: Vec<_> = vectors
        .iter()
        .map(|(request, _)| classify(validate(&state, request)))
        .collect();

    for _ in 0..64 {
        let actual: Vec<_> = vectors
            .iter()
            .map(|(request, expected)| {
                assert_eq!(classify(validate(&state, request)), *expected);
                classify(validate(&state, request))
            })
            .collect();
        assert_eq!(actual, expected);
    }
    Ok(())
}
