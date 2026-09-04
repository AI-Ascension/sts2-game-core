// SPDX-License-Identifier: MIT

//! Pure semantic values and validation for the STS2 domain boundary.

mod calculators;
mod combat;
mod end_turn;
mod identity;
mod play_card;
mod probability;
mod protocol_artifact;
mod runtime_v2;
mod simulator;
mod state;
mod validation;

pub use calculators::{
    CalculatorError, CombatCalculationState, EnemyFacts, ExactDamage, ExactResourceResult,
    ExactSurvival, exact_card_damage, exact_end_turn_survival, exact_lethal,
    exact_resource_after_card,
};
pub use combat::{CombatPhase, CombatSnapshot, TurnIndex};
pub use end_turn::{
    EndTurnAction, EndTurnApplyError, EndTurnEffectWitness, EndTurnRequest, EndTurnValidationError,
    SettledEndTurn, ValidatedEndTurn, validate_end_turn,
};
pub use identity::{Generation, Identity, SessionId};
pub use play_card::{
    CardSpec, CardTarget, PlayCardFacts, PlayCardRequest, PlayCardValidationError,
    TargetDomain, ValidatedPlayCard, calculate_play_card, validate_play_card,
};
pub use probability::{
    BeliefOutcome, BeliefState, EstimateSource, ProbabilityEstimate,
};
pub use protocol_artifact::{
    ArtifactError, POC_ARTIFACT, POC_PROTOCOL_VERSION, POC_SCHEMA_DIGEST, verify_poc_artifact,
};
pub use runtime_v2::{
    RUNTIME_V2_MAX_GENERATION, RUNTIME_V2_MAX_TURN_INDEX, RuntimeV2Generation,
    RuntimeV2Observation, RuntimeV2ProjectionError, RuntimeV2TurnIndex,
};
pub use state::{Action, ActionId, ApplyError, Phase, Request, State, ValidatedAction};
pub use simulator::{SimulationSummary, simulate_end_turn};
pub use validation::{ValidationError, validate};
