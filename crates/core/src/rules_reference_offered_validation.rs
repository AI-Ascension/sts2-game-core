// SPDX-License-Identifier: MIT

//! Validation for the host-chosen offered set.
//!
//! These checks confirm that a set of offered entries is internally consistent as data the host
//! already chose. They say nothing about how the host chose it: a set may pass every check here
//! and still be an outcome this target could never have predicted, because generation weights,
//! rerolls, and undisclosed outcomes are not represented.

use std::fmt;

use super::rules_reference_offered::{
    MAX_OFFERED_ATTRIBUTE_BYTES, OfferedAttribute, OfferedAttributeName, OfferedEntry,
    OfferedEntryIdentity, OfferedEntryKind,
};

/// Why one offered entry or offered set was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OfferedSetError {
    /// One identity was empty, which is an absent value rather than an identity.
    EmptyIdentity { kind: OfferedEntryKind },
    /// One identity appeared more than once in the offered set.
    DuplicateIdentity {
        kind: OfferedEntryKind,
        position: usize,
    },
    /// One disclosed attribute exceeded [`MAX_OFFERED_ATTRIBUTE_BYTES`].
    AttributeTooLong {
        kind: OfferedEntryKind,
        attribute: OfferedAttributeName,
    },
}

impl fmt::Display for OfferedSetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentity { kind } => {
                write!(
                    formatter,
                    "offered set has an empty {} identity",
                    kind.as_str()
                )
            }
            Self::DuplicateIdentity { kind, position } => {
                write!(
                    formatter,
                    "offered set repeats a {} identity at position {position}",
                    kind.as_str()
                )
            }
            Self::AttributeTooLong { kind, attribute } => {
                write!(
                    formatter,
                    "offered {} {} attribute exceeds {MAX_OFFERED_ATTRIBUTE_BYTES} bytes",
                    kind.as_str(),
                    attribute.as_str()
                )
            }
        }
    }
}

impl std::error::Error for OfferedSetError {}

/// Builds one offered entry from an identity and the attributes the host supplied.
///
/// Each attribute is passed independently, so an undisclosed rarity or description stays absent
/// rather than being replaced with a value this target has no source for.
#[must_use]
pub const fn offered_entry<'a>(
    kind: OfferedEntryKind,
    id: &'a str,
    name: OfferedAttribute<&'a str>,
    rarity: OfferedAttribute<&'a str>,
    description: OfferedAttribute<&'a str>,
) -> OfferedEntry<'a> {
    OfferedEntry {
        identity: OfferedEntryIdentity::new(kind, id),
        name,
        rarity,
        description,
    }
}

/// Checks one entry's identity and disclosed attributes.
///
/// # Errors
///
/// Returns [`OfferedSetError::EmptyIdentity`] when the identity is empty, and
/// [`OfferedSetError::AttributeTooLong`] when a disclosed attribute exceeds
/// [`MAX_OFFERED_ATTRIBUTE_BYTES`]. An undisclosed attribute is never defaulted, so it cannot
/// fail here.
pub fn validate_offered_entry(entry: &OfferedEntry<'_>) -> Result<(), OfferedSetError> {
    if entry.identity.id.is_empty() {
        return Err(OfferedSetError::EmptyIdentity {
            kind: entry.identity.kind,
        });
    }
    for (attribute, name) in [
        (entry.name, OfferedAttributeName::Name),
        (entry.rarity, OfferedAttributeName::Rarity),
        (entry.description, OfferedAttributeName::Description),
    ] {
        if let Some(value) = attribute.supplied()
            && value.len() > MAX_OFFERED_ATTRIBUTE_BYTES
        {
            return Err(OfferedSetError::AttributeTooLong {
                kind: entry.identity.kind,
                attribute: name,
            });
        }
    }
    Ok(())
}

/// Checks one offered set for repeated identities, empty identities, and oversized attributes.
///
/// Duplicate detection is by identity namespace *and* value, so the same identifier string offered
/// once as a card and once as a reward is two entries rather than a duplicate.
///
/// # Errors
///
/// Returns the first [`OfferedSetError`] in entry order.
pub fn validate_offered_set(entries: &[OfferedEntry<'_>]) -> Result<(), OfferedSetError> {
    let mut seen: Vec<(OfferedEntryKind, &str)> = Vec::with_capacity(entries.len());
    for (position, entry) in entries.iter().enumerate() {
        validate_offered_entry(entry)?;
        let identity = (entry.identity.kind, entry.identity.id);
        if seen.contains(&identity) {
            return Err(OfferedSetError::DuplicateIdentity {
                kind: entry.identity.kind,
                position,
            });
        }
        seen.push(identity);
    }
    Ok(())
}

/// Returns the number of entries whose disclosed attributes are complete.
#[must_use]
pub fn count_fully_described(entries: &[OfferedEntry<'_>]) -> usize {
    entries
        .iter()
        .filter(|entry| {
            entry.name.is_supplied()
                && entry.rarity.is_supplied()
                && entry.description.is_supplied()
        })
        .count()
}
