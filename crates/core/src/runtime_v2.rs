// SPDX-License-Identifier: MIT

use std::convert::TryFrom;

use crate::combat::{CombatPhase, CombatSnapshot, TurnIndex};
use crate::identity::{Generation, Identity, SessionId};

/// The inclusive maximum turn index representable by Runtime-v2.
pub const RUNTIME_V2_MAX_TURN_INDEX: u32 = 1_024;
/// The inclusive maximum generation representable as a Runtime-v2 safe integer.
pub const RUNTIME_V2_MAX_GENERATION: u64 = 9_007_199_254_740_991;

/// A turn index proven safe for Runtime-v2 representation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RuntimeV2TurnIndex(u32);

impl RuntimeV2TurnIndex {
    /// Checks and constructs a numeric turn index for Runtime-v2 representation.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeV2ProjectionError::TurnIndexOutOfRange`] when `value` exceeds the
    /// inclusive Runtime-v2 maximum.
    pub const fn try_from_value(value: u32) -> Result<Self, RuntimeV2ProjectionError> {
        if value <= RUNTIME_V2_MAX_TURN_INDEX {
            Ok(Self(value))
        } else {
            Err(RuntimeV2ProjectionError::TurnIndexOutOfRange {
                value,
                max: RUNTIME_V2_MAX_TURN_INDEX,
            })
        }
    }

    /// Returns the checked numeric turn index.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl TryFrom<TurnIndex> for RuntimeV2TurnIndex {
    type Error = RuntimeV2ProjectionError;

    fn try_from(value: TurnIndex) -> Result<Self, Self::Error> {
        Self::try_from_value(value.value())
    }
}

/// A generation proven safe for Runtime-v2 numeric representation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RuntimeV2Generation(u64);

impl RuntimeV2Generation {
    /// Checks and constructs a numeric generation for Runtime-v2 representation.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeV2ProjectionError::GenerationOutOfRange`] when `value` exceeds the
    /// inclusive Runtime-v2 safe-integer maximum.
    pub const fn try_from_value(value: u64) -> Result<Self, RuntimeV2ProjectionError> {
        if value <= RUNTIME_V2_MAX_GENERATION {
            Ok(Self(value))
        } else {
            Err(RuntimeV2ProjectionError::GenerationOutOfRange {
                value,
                max: RUNTIME_V2_MAX_GENERATION,
            })
        }
    }

    /// Returns the checked numeric generation.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl TryFrom<Generation> for RuntimeV2Generation {
    type Error = RuntimeV2ProjectionError;

    fn try_from(value: Generation) -> Result<Self, Self::Error> {
        Self::try_from_value(value.value())
    }
}

/// A deterministic failure while projecting a domain value into Runtime-v2 representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeV2ProjectionError {
    /// The domain turn index exceeds the Runtime-v2 inclusive maximum.
    TurnIndexOutOfRange {
        /// The unrepresentable domain value.
        value: u32,
        /// The inclusive Runtime-v2 maximum.
        max: u32,
    },
    /// The domain generation exceeds the Runtime-v2 safe-integer maximum.
    GenerationOutOfRange {
        /// The unrepresentable domain value.
        value: u64,
        /// The inclusive Runtime-v2 maximum.
        max: u64,
    },
}

/// A Runtime-v2 observation containing only checked numeric representations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeV2Observation {
    actor: Identity,
    session: SessionId,
    generation: RuntimeV2Generation,
    phase: CombatPhase,
    turn_index: RuntimeV2TurnIndex,
}

impl RuntimeV2Observation {
    /// Returns the actor carried by the checked observation.
    #[must_use]
    pub const fn actor(self) -> Identity {
        self.actor
    }

    /// Returns the session carried by the checked observation.
    #[must_use]
    pub const fn session(self) -> SessionId {
        self.session
    }

    /// Returns the checked Runtime-v2 generation.
    #[must_use]
    pub const fn generation(self) -> RuntimeV2Generation {
        self.generation
    }

    /// Returns the combat phase carried by the checked observation.
    #[must_use]
    pub const fn phase(self) -> CombatPhase {
        self.phase
    }

    /// Returns the checked Runtime-v2 turn index.
    #[must_use]
    pub const fn turn_index(self) -> RuntimeV2TurnIndex {
        self.turn_index
    }
}

impl TryFrom<CombatSnapshot> for RuntimeV2Observation {
    type Error = RuntimeV2ProjectionError;

    fn try_from(snapshot: CombatSnapshot) -> Result<Self, Self::Error> {
        Ok(Self {
            actor: snapshot.actor(),
            session: snapshot.session(),
            generation: RuntimeV2Generation::try_from(snapshot.generation())?,
            phase: snapshot.phase(),
            turn_index: RuntimeV2TurnIndex::try_from(snapshot.turn_index())?,
        })
    }
}

impl Generation {
    /// Checks this generation before using it in a Runtime-v2 representation.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeV2ProjectionError::GenerationOutOfRange`] when this generation exceeds
    /// the inclusive Runtime-v2 safe-integer maximum.
    pub const fn try_runtime_v2(self) -> Result<RuntimeV2Generation, RuntimeV2ProjectionError> {
        RuntimeV2Generation::try_from_value(self.value())
    }
}

impl TurnIndex {
    /// Checks this turn index before using it in a Runtime-v2 representation.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeV2ProjectionError::TurnIndexOutOfRange`] when this turn index exceeds the
    /// inclusive Runtime-v2 maximum.
    pub const fn try_runtime_v2(self) -> Result<RuntimeV2TurnIndex, RuntimeV2ProjectionError> {
        RuntimeV2TurnIndex::try_from_value(self.value())
    }
}

impl CombatSnapshot {
    /// Projects this snapshot into a checked Runtime-v2 observation.
    ///
    /// The projection is read-only and returns an error before producing an observation whenever
    /// either numeric field exceeds its Runtime-v2 representation limit.
    ///
    /// # Errors
    ///
    /// Returns a [`RuntimeV2ProjectionError`] when the generation or turn index is out of range.
    pub fn try_runtime_v2(self) -> Result<RuntimeV2Observation, RuntimeV2ProjectionError> {
        RuntimeV2Observation::try_from(self)
    }
}
