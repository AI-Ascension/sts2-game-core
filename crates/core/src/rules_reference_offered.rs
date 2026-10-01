// SPDX-License-Identifier: MIT

//! Pure typed offered entries for the host-chosen acquisition set.
//!
//! The host has already chosen these entries at offer time. This module describes what the host
//! supplied and nothing more: an entry's identity, its name, and the attributes the host actually
//! carried. An attribute the host did not disclose stays absent, because this target has no source
//! for a value the host withheld, and a second, unverified account of the game in front of a
//! consumer is worse than an absent field.
//!
//! Generation policy is deliberately inexpressible here. There is no probability type, no weight,
//! no reroll rate, and no undisclosed outcome in this module: the offered set's *contents* are
//! given, and how the host came to choose them is not represented. A rarity that is present is the
//! host's own spelling, carried verbatim and never derived, defaulted, or ranked.

use std::fmt;

use super::rules_reference_vocab::RuleUnit;

/// Largest offered-entry attribute length this module accepts.
///
/// This reuses the harness `MAX_DESCRIPTION_BYTES` value (240) so an admitted
/// attribute is never truncated against the harness's own limit. The two bounds
/// are not equivalent, though, and the difference is deliberate rather than
/// accidental: the harness constant bounds the WHOLE composed phrase for one
/// entry (name, upgraded marker, cost, rarity and description together), while
/// this constant bounds EACH attribute independently. Three attributes can
/// therefore each reach 240 here even though no single harness entry could.
/// This is a safe over-estimate — it refuses less, never more — and it is stated
/// explicitly so a future reader does not mistake the shared number for a shared
/// accounting.
pub const MAX_OFFERED_ATTRIBUTE_BYTES: usize = 240;

/// One offered-entry attribute, either as the host supplied it or as undisclosed.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum OfferedAttribute<T> {
    /// The host supplied the attribute in this same observation.
    Supplied(T),
    /// The host did not disclose the attribute; it stays absent rather than being defaulted.
    Undisclosed,
}

impl<T> OfferedAttribute<T> {
    /// Returns the host-supplied value, or `None` when the attribute was not disclosed.
    ///
    /// This is the only accessor that produces an attribute value, so an undisclosed attribute
    /// cannot be observed as a default through any other route.
    #[must_use]
    pub fn supplied(self) -> Option<T> {
        match self {
            Self::Supplied(value) => Some(value),
            Self::Undisclosed => None,
        }
    }

    /// Returns whether the host disclosed this attribute.
    #[must_use]
    pub fn is_supplied(self) -> bool {
        matches!(self, Self::Supplied(_))
    }

    /// Returns whether this attribute stayed absent.
    #[must_use]
    pub fn is_undisclosed(self) -> bool {
        matches!(self, Self::Undisclosed)
    }

    /// Maps a disclosed value, leaving an undisclosed attribute absent.
    #[must_use]
    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> Option<U> {
        match self {
            Self::Supplied(value) => Some(map(value)),
            Self::Undisclosed => None,
        }
    }
}

impl<T> From<Option<T>> for OfferedAttribute<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::Supplied(value),
            None => Self::Undisclosed,
        }
    }
}

impl<T> Default for OfferedAttribute<T> {
    /// Returns the undisclosed attribute, so an omitted attribute defaults to absence, never to a
    /// value.
    fn default() -> Self {
        Self::Undisclosed
    }
}

impl<T: fmt::Debug> fmt::Debug for OfferedAttribute<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Supplied(value) => formatter.debug_tuple("Supplied").field(value).finish(),
            Self::Undisclosed => formatter.write_str("Undisclosed"),
        }
    }
}

/// The namespace one offered entry's identity was drawn from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OfferedEntryKind {
    /// A card the host offered; identity is the host's own card identity.
    Card,
    /// A reward container the host offered, such as a card reward that opens a later screen.
    Reward,
    /// An item the host offered in a shop.
    ShopItem,
}

impl OfferedEntryKind {
    /// Returns the namespace field the host uses for this identity.
    #[must_use]
    pub const fn identity_field(self) -> &'static str {
        match self {
            Self::Card => "card_id",
            Self::Reward => "reward_id",
            Self::ShopItem => "item_id",
        }
    }

    /// Returns the transport-neutral stable key.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Card => "card",
            Self::Reward => "reward",
            Self::ShopItem => "shop_item",
        }
    }
}

/// One offered entry's identity and its namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfferedEntryIdentity<'a> {
    pub kind: OfferedEntryKind,
    pub id: &'a str,
}

impl<'a> OfferedEntryIdentity<'a> {
    /// Wraps one host-supplied identity in its namespace.
    #[must_use]
    pub const fn new(kind: OfferedEntryKind, id: &'a str) -> Self {
        Self { kind, id }
    }
}

/// One offered entry's identity and the attributes the host actually supplied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfferedEntry<'a> {
    pub identity: OfferedEntryIdentity<'a>,
    pub name: OfferedAttribute<&'a str>,
    pub rarity: OfferedAttribute<&'a str>,
    pub description: OfferedAttribute<&'a str>,
}

impl OfferedEntry<'_> {
    /// Declares which of this entry's attributes the host supplied.
    ///
    /// Every declared attribute is present in the returned set, so a consumer can record what was
    /// disclosed without inspecting which attributes this target happens to model.
    #[must_use]
    pub fn disclosed_attributes(self) -> &'static [OfferedAttributeName] {
        match (
            self.name.is_supplied(),
            self.rarity.is_supplied(),
            self.description.is_supplied(),
        ) {
            (true, true, true) => &[
                OfferedAttributeName::Name,
                OfferedAttributeName::Rarity,
                OfferedAttributeName::Description,
            ],
            (true, true, false) => &[OfferedAttributeName::Name, OfferedAttributeName::Rarity],
            (true, false, true) => &[
                OfferedAttributeName::Name,
                OfferedAttributeName::Description,
            ],
            (true, false, false) => &[OfferedAttributeName::Name],
            (false, true, true) => &[
                OfferedAttributeName::Rarity,
                OfferedAttributeName::Description,
            ],
            (false, true, false) => &[OfferedAttributeName::Rarity],
            (false, false, true) => &[OfferedAttributeName::Description],
            (false, false, false) => &[],
        }
    }

    /// Returns the documented rules-reference unit of each disclosed attribute.
    ///
    /// A host rarity is dimensionless text, never a numeric or weighted unit, so this method
    /// cannot report a probability-bearing unit for any attribute this target models.
    #[must_use]
    pub fn attribute_units(self) -> Vec<(OfferedAttributeName, RuleUnit)> {
        self.disclosed_attributes()
            .iter()
            .map(|name| (*name, name.rule_unit()))
            .collect()
    }
}

/// One modelled offered-entry attribute.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OfferedAttributeName {
    /// The host's own display name for this entry.
    Name,
    /// The host's own rarity spelling for this entry.
    Rarity,
    /// The host's own text for what this entry is.
    Description,
}

impl OfferedAttributeName {
    /// Returns the transport-neutral stable key.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Rarity => "rarity",
            Self::Description => "description",
        }
    }

    /// Returns the declared rules-reference unit for this attribute.
    ///
    /// Host rarity is dimensionless text rather than a probability, so it maps to
    /// [`RuleUnit::Dimensionless`] and never to a numeric or weighted unit.
    #[must_use]
    pub const fn rule_unit(self) -> RuleUnit {
        match self {
            Self::Name | Self::Rarity | Self::Description => RuleUnit::Dimensionless,
        }
    }
}
