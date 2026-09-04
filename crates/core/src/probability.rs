// SPDX-License-Identifier: MIT

use crate::calculators::{CalculatorError, CombatCalculationState, ExactSurvival, exact_end_turn_survival};

const MAX_BELIEF_OUTCOMES: usize = 256;

/// One explicitly supplied possible observation and its positive integer weight.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BeliefOutcome {
    pub state: CombatCalculationState,
    pub weight: u32,
}

/// A bounded belief distribution. It has no seed or access to future game randomness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BeliefState {
    outcomes: Vec<BeliefOutcome>,
    total_weight: u64,
}

impl BeliefState {
    /// Creates a belief distribution from explicit, validated outcomes.
    pub fn new(outcomes: Vec<BeliefOutcome>) -> Result<Self, CalculatorError> {
        if outcomes.is_empty() || outcomes.len() > MAX_BELIEF_OUTCOMES {
            return Err(CalculatorError::MalformedObservation);
        }
        let mut total_weight = 0_u64;
        for outcome in &outcomes {
            outcome.state.validate()?;
            if outcome.weight == 0 {
                return Err(CalculatorError::MalformedObservation);
            }
            total_weight = total_weight
                .checked_add(u64::from(outcome.weight))
                .ok_or(CalculatorError::ArithmeticOverflow)?;
        }
        Ok(Self {
            outcomes,
            total_weight,
        })
    }

    #[must_use]
    pub fn outcomes(&self) -> &[BeliefOutcome] {
        &self.outcomes
    }

    #[must_use]
    pub const fn total_weight(&self) -> u64 {
        self.total_weight
    }

    /// Evaluates exact survival over only the explicitly supplied belief states.
    pub fn survival_estimate(&self) -> Result<ProbabilityEstimate, CalculatorError> {
        let mut surviving_weight = 0_u64;
        for outcome in &self.outcomes {
            if exact_end_turn_survival(&outcome.state)?.survives {
                surviving_weight = surviving_weight
                    .checked_add(u64::from(outcome.weight))
                    .ok_or(CalculatorError::ArithmeticOverflow)?;
            }
        }
        Ok(ProbabilityEstimate {
            successful_weight: surviving_weight,
            total_weight: self.total_weight,
            source: EstimateSource::ExplicitBelief,
        })
    }
}

/// Evidence label that prevents an estimate from being mistaken for an exact fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstimateSource {
    ExplicitBelief,
}

/// Rational estimate represented without floating-point rounding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProbabilityEstimate {
    pub successful_weight: u64,
    pub total_weight: u64,
    pub source: EstimateSource,
}

impl ProbabilityEstimate {
    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.successful_weight
    }

    #[must_use]
    pub const fn denominator(self) -> u64 {
        self.total_weight
    }
}

/// Exposes exact facts for each explicit branch to the simulator.
pub(crate) fn survival_for_state(
    state: &CombatCalculationState,
) -> Result<ExactSurvival, CalculatorError> {
    exact_end_turn_survival(state)
}
