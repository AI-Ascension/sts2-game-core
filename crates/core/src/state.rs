// SPDX-License-Identifier: MIT

use crate::identity::{Generation, Identity};

/// The lifecycle phase relevant to the initial semantic validation seam.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    /// Requests may be validated against the current state.
    Open,
    /// Requests cannot be accepted after closure.
    Closed,
}

/// A typed, side-effect-free action proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Requests use of a bounded number of available units.
    UseBudget { units: u16 },
    /// Requests closure of an open state.
    Close,
}

/// An immutable point-in-time state used as the validation authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct State {
    owner: Identity,
    generation: Generation,
    phase: Phase,
    available_units: u16,
}

impl State {
    /// Creates a state snapshot with explicit identity, generation, phase, and capacity.
    #[must_use]
    pub const fn new(
        owner: Identity,
        generation: Generation,
        phase: Phase,
        available_units: u16,
    ) -> Self {
        Self {
            owner,
            generation,
            phase,
            available_units,
        }
    }

    /// Returns the identity that owns this snapshot.
    #[must_use]
    pub const fn owner(self) -> Identity {
        self.owner
    }

    /// Returns the snapshot generation used for freshness checks.
    #[must_use]
    pub const fn generation(self) -> Generation {
        self.generation
    }

    /// Returns the lifecycle phase in this snapshot.
    #[must_use]
    pub const fn phase(self) -> Phase {
        self.phase
    }

    /// Returns the currently available bounded units.
    #[must_use]
    pub const fn available_units(self) -> u16 {
        self.available_units
    }
}

/// A request carrying the actor and snapshot generation it was based on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Request {
    actor: Identity,
    expected_generation: Generation,
    action: Action,
}

impl Request {
    /// Creates a typed request for validation against a state snapshot.
    #[must_use]
    pub const fn new(actor: Identity, expected_generation: Generation, action: Action) -> Self {
        Self {
            actor,
            expected_generation,
            action,
        }
    }

    /// Returns the identity that proposed the action.
    #[must_use]
    pub const fn actor(self) -> Identity {
        self.actor
    }

    /// Returns the generation the proposer observed.
    #[must_use]
    pub const fn expected_generation(self) -> Generation {
        self.expected_generation
    }

    /// Returns the typed action proposal.
    #[must_use]
    pub const fn action(self) -> Action {
        self.action
    }
}

/// A request that passed all core validation checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    actor: Identity,
    generation: Generation,
    action: Action,
}

impl ValidatedAction {
    /// Returns the actor associated with the accepted proposal.
    #[must_use]
    pub const fn actor(self) -> Identity {
        self.actor
    }

    /// Returns the state generation against which the proposal passed.
    #[must_use]
    pub const fn generation(self) -> Generation {
        self.generation
    }

    /// Returns the accepted action without performing it.
    #[must_use]
    pub const fn action(self) -> Action {
        self.action
    }

    pub(crate) const fn from_request(request: Request) -> Self {
        Self {
            actor: request.actor(),
            generation: request.expected_generation(),
            action: request.action(),
        }
    }
}
