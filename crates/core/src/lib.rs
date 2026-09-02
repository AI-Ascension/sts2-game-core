// SPDX-License-Identifier: MIT

//! Pure semantic values and validation for the STS2 domain boundary.

mod combat;
mod end_turn;
mod identity;
mod protocol_artifact;
mod state;
mod validation;

pub use combat::{CombatPhase, CombatSnapshot, TurnIndex};
pub use end_turn::{
    EndTurnAction, EndTurnApplyError, EndTurnEffectWitness, EndTurnRequest, EndTurnValidationError,
    SettledEndTurn, ValidatedEndTurn, validate_end_turn,
};
pub use identity::{Generation, Identity, SessionId};
pub use protocol_artifact::{
    ArtifactError, POC_ARTIFACT, POC_PROTOCOL_VERSION, POC_SCHEMA_DIGEST, verify_poc_artifact,
};
pub use state::{Action, ActionId, ApplyError, Phase, Request, State, ValidatedAction};
pub use validation::{ValidationError, validate};
