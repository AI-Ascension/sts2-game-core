// SPDX-License-Identifier: MIT

use crate::state::{Action, Request, State, ValidatedAction};

/// A deterministic reason that a typed request was not accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationError {
    /// The request actor does not own the supplied state snapshot.
    ActorMismatch {
        /// The owner recorded by the state snapshot.
        expected: crate::Identity,
        /// The actor carried by the request.
        actual: crate::Identity,
    },
    /// The request was based on a different point-in-time snapshot.
    StaleGeneration {
        /// The generation carried by the request.
        expected: crate::Generation,
        /// The generation currently supplied for validation.
        actual: crate::Generation,
    },
    /// A budget action must request at least one unit.
    ZeroUnits,
    /// A budget action requested more units than the snapshot provides.
    InsufficientUnits {
        /// The requested number of units.
        requested: u16,
        /// The number of units available in the snapshot.
        available: u16,
    },
    /// The snapshot is closed and cannot accept another action.
    ClosedState,
}

/// Validates a request against one immutable snapshot without changing either value.
///
/// Checks occur in this order: actor, generation, action argument bounds, lifecycle phase, and
/// available capacity. A successful result is an accepted proposal, not proof of execution.
///
/// # Errors
///
/// Returns a [`ValidationError`] describing the first failed check.
pub fn validate(state: &State, request: &Request) -> Result<ValidatedAction, ValidationError> {
    if request.actor() != state.owner() {
        return Err(ValidationError::ActorMismatch {
            expected: state.owner(),
            actual: request.actor(),
        });
    }
    if request.expected_generation() != state.generation() {
        return Err(ValidationError::StaleGeneration {
            expected: request.expected_generation(),
            actual: state.generation(),
        });
    }

    match request.action() {
        Action::UseBudget { units: 0 } => Err(ValidationError::ZeroUnits),
        Action::UseBudget { units } if state.phase() == crate::Phase::Closed => {
            let _ = units;
            Err(ValidationError::ClosedState)
        }
        Action::UseBudget { units } if units > state.available_units() => {
            Err(ValidationError::InsufficientUnits {
                requested: units,
                available: state.available_units(),
            })
        }
        Action::UseBudget { .. } | Action::Close if state.phase() == crate::Phase::Open => {
            Ok(ValidatedAction::from_request(*request))
        }
        Action::Close | Action::UseBudget { .. } => Err(ValidationError::ClosedState),
    }
}
