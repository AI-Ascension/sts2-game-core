// SPDX-License-Identifier: MIT

//! Pure semantic values and validation for the STS2 domain boundary.

mod identity;
mod protocol_artifact;
mod state;
mod validation;

pub use identity::{Generation, Identity};
pub use protocol_artifact::{
    POC_ARTIFACT, POC_PROTOCOL_VERSION, POC_SCHEMA_DIGEST, verify_poc_artifact,
};
pub use state::{Action, ActionId, ApplyError, Phase, Request, State, ValidatedAction};
pub use validation::{ValidationError, validate};
