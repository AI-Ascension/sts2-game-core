// SPDX-License-Identifier: MIT

use crate::calculators::{CalculatorError, ExactSurvival};
use crate::probability::{BeliefState, EstimateSource, survival_for_state};

/// Aggregated result of enumerating an explicit belief distribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulationSummary {
    pub total_weight: u64,
    pub surviving_weight: u64,
    pub hp_after_weighted_sum: u64,
    pub source: EstimateSource,
}

/// Evaluates every explicit branch once; it never predicts hidden future randomness.
pub fn simulate_end_turn(belief: &BeliefState) -> Result<SimulationSummary, CalculatorError> {
    let mut surviving_weight = 0_u64;
    let mut hp_after_weighted_sum = 0_u64;
    for outcome in belief.outcomes() {
        let survival: ExactSurvival = survival_for_state(&outcome.state)?;
        if survival.survives {
            surviving_weight = surviving_weight
                .checked_add(u64::from(outcome.weight))
                .ok_or(CalculatorError::ArithmeticOverflow)?;
        }
        hp_after_weighted_sum = hp_after_weighted_sum
            .checked_add(u64::from(survival.hp_after) * u64::from(outcome.weight))
            .ok_or(CalculatorError::ArithmeticOverflow)?;
    }
    Ok(SimulationSummary {
        total_weight: belief.total_weight(),
        surviving_weight,
        hp_after_weighted_sum,
        source: EstimateSource::ExplicitBelief,
    })
}
