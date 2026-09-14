// SPDX-License-Identifier: MIT

//! Pure semantic values and validation for the STS2 domain boundary.

mod calculators;
mod combat;
mod end_turn;
mod identity;
mod play_card;
mod probability;
mod protocol_artifact;
mod rules_reference;
mod rules_reference_catalog;
mod rules_reference_coverage;
mod rules_reference_fixtures;
mod rules_reference_ids;
mod rules_reference_index;
mod rules_reference_lookup;
mod rules_reference_query;
mod rules_reference_records;
mod rules_reference_records_combat;
mod rules_reference_records_defense;
mod rules_reference_records_flow;
mod rules_reference_records_interactions;
mod rules_reference_records_scaling;
mod rules_reference_records_support;
mod rules_reference_vocab;
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
    CardSpec, CardTarget, PlayCardFacts, PlayCardRequest, PlayCardValidationError, TargetDomain,
    ValidatedPlayCard, calculate_play_card, validate_play_card,
};
pub use probability::{BeliefOutcome, BeliefState, EstimateSource, ProbabilityEstimate};
pub use protocol_artifact::{
    ArtifactError, POC_ARTIFACT, POC_PROTOCOL_VERSION, POC_SCHEMA_DIGEST, verify_poc_artifact,
};
pub use rules_reference::{
    BuildScope, ContentScope, EvidenceStatus, GameMode, MAX_RULE_MATCHES, RULES_REFERENCE_VERSION,
    RuleApplicability, RuleContext, RuleReference, RuleSupport, SourceStatus,
};
pub use rules_reference_coverage::UnmodeledCombination;
pub use rules_reference_fixtures::{
    ASCENSION_ENEMY_HP_FIXTURE, BLOCK_EXPIRY_CLAMPED_FIXTURE, BLOCK_EXPIRY_FIXTURE,
    CARD_MULTI_HIT_ORDERING_FIXTURE, COOP_ENEMY_HP_FIXTURE, DRAW_TO_HAND_SIZE_FIXTURE,
    DRAW_WITH_FULL_HAND_FIXTURE, ENERGY_CAP_FIXTURE, EXHAUST_ZONE_MOVE_FIXTURE, FixtureClass,
    FixtureOp, ORDERED_FIXTURES, OrderedFixture, POTION_CONSUMPTION_FIXTURE,
    RELIC_FLAT_BONUS_ORDERING_FIXTURE, REWARD_CHOICE_FIXTURE,
    REWARD_CHOICE_OUT_OF_COVERAGE_FIXTURE, SATURATING_HEAL_FIXTURE,
    SIMPLIFIED_NOMINAL_DAMAGE_FIXTURE, START_OF_TURN_ENERGY_FIXTURE, STATUS_VULNERABLE_FIXTURE,
    SYNTHETIC_MODIFIER_FIXTURE, SimplifiedEstimateFixture, SyntheticDamageFixture,
};
pub use rules_reference_ids::RuleId;
pub use rules_reference_lookup::{
    coverage_for, coverage_inventory, lookup, lookup_rules, rule_inventory, rules_for,
    rules_for_entity, rules_for_mechanic, unmodeled_combinations, unmodeled_for,
};
pub use rules_reference_query::{
    RuleCollectionLookup, RuleCoverageStatus, RuleFamilyCoverage, RuleLookup, RuleMatches,
    RuleQuery,
};
pub use rules_reference_vocab::{
    EntityKind, EntityReference, ExpiryRule, Mechanic, RoundingRule, RuleFamily, RuleInput,
    RuleOperation, RuleStep, RuleStepKind, RuleUnit, StackRule, TargetingRule,
};
pub use runtime_v2::{
    RUNTIME_V2_MAX_GENERATION, RUNTIME_V2_MAX_TURN_INDEX, RuntimeV2Generation,
    RuntimeV2Observation, RuntimeV2ProjectionError, RuntimeV2TurnIndex,
};
pub use simulator::{SimulationSummary, simulate_end_turn};
pub use state::{Action, ActionId, ApplyError, Phase, Request, State, ValidatedAction};
pub use validation::{ValidationError, validate};
