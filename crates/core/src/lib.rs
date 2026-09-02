// SPDX-License-Identifier: MIT

//! Pure semantic values and validation for the STS2 domain boundary.

mod identity;
mod state;
mod validation;

pub use identity::{Generation, Identity};
pub use state::{Action, Phase, Request, State, ValidatedAction};
pub use validation::{ValidationError, validate};
