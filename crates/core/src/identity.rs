// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

/// A stable non-zero identity for the actor addressed by a semantic request.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identity(NonZeroU64);

impl Identity {
    /// Creates an identity, rejecting the reserved zero value.
    #[must_use]
    pub fn new(value: u64) -> Option<Self> {
        NonZeroU64::new(value).map(Self)
    }

    /// Returns the stable numeric representation used by this domain boundary.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

/// A monotonically increasing version for a point-in-time semantic state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Generation(u64);

impl Generation {
    /// Returns the first generation in a state history.
    #[must_use]
    pub const fn initial() -> Self {
        Self(0)
    }

    /// Wraps an externally supplied, already typed generation value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric value of this generation.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Advances one generation, returning None instead of wrapping at the maximum value.
    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Generation, Identity};

    #[test]
    fn zero_is_not_an_identity() {
        assert_eq!(Identity::new(0), None);
        assert_eq!(Identity::new(9).map(Identity::value), Some(9));
    }

    #[test]
    fn generation_advancement_is_checked() {
        assert_eq!(Generation::initial().value(), 0);
        assert_eq!(Generation::new(4).next(), Some(Generation::new(5)));
        assert_eq!(Generation::new(u64::MAX).next(), None);
    }
}
