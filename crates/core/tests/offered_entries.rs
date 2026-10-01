// SPDX-License-Identifier: MIT

//! Tests for the host-chosen offered entries admitted to the `Acquisition` domain.
//!
//! These tests bind three properties of the modelled offered set: an identity may not repeat
//! inside one set, an attribute the host withheld stays absent, and a rarity is only ever the
//! host's own spelling. No test here invents a probability, a weight, or a reroll rate, and none
//! can: the modelled types have no field to hold one.

use sts2_game_core::{
    MAX_OFFERED_ATTRIBUTE_BYTES, OfferedAttribute, OfferedAttributeName, OfferedEntryKind,
    OfferedSetError, RuleFamily, RuleId, RuleUnit, count_fully_described, coverage_for,
    offered_entry, unmodeled_combinations, unmodeled_for, validate_offered_entry,
    validate_offered_set,
};

/// One entry with every attribute the host supplied.
fn fully_described<'a>(id: &'a str, rarity: &'a str) -> sts2_game_core::OfferedEntry<'a> {
    offered_entry(
        OfferedEntryKind::Card,
        id,
        OfferedAttribute::Supplied("Strike"),
        OfferedAttribute::Supplied(rarity),
        OfferedAttribute::Supplied("deal 6 damage"),
    )
}

/// One entry whose rarity the host did not disclose.
fn rarity_undisclosed(id: &str) -> sts2_game_core::OfferedEntry<'_> {
    offered_entry(
        OfferedEntryKind::Card,
        id,
        OfferedAttribute::Supplied("Strike"),
        OfferedAttribute::Undisclosed,
        OfferedAttribute::Supplied("deal 6 damage"),
    )
}

#[test]
fn a_repeated_identity_is_rejected_while_distinct_namespaces_are_not() {
    let duplicate = [
        fully_described("card.definition", "common"),
        fully_described("card.definition", "uncommon"),
    ];
    assert_eq!(
        validate_offered_set(&duplicate),
        Err(OfferedSetError::DuplicateIdentity {
            kind: OfferedEntryKind::Card,
            position: 1,
        })
    );
    assert_eq!(
        validate_offered_set(&duplicate).unwrap_err().to_string(),
        "offered set repeats a card identity at position 1"
    );

    // The same identifier string in two namespaces is two host-chosen entries, not a duplicate.
    let distinct = [
        fully_described("shared.definition", "common"),
        offered_entry(
            OfferedEntryKind::Reward,
            "shared.definition",
            OfferedAttribute::Supplied("Card Reward"),
            OfferedAttribute::Undisclosed,
            OfferedAttribute::Undisclosed,
        ),
    ];
    assert_eq!(validate_offered_set(&distinct), Ok(()));
    assert_eq!(count_fully_described(&distinct), 1);
    assert_eq!(distinct[1].identity.kind, OfferedEntryKind::Reward);
    assert_eq!(distinct[1].identity.id, "shared.definition");
}

#[test]
fn an_undisclosed_attribute_stays_absent_instead_of_being_defaulted() {
    let entry = rarity_undisclosed("card.definition");
    assert!(entry.rarity.is_undisclosed());
    assert!(!entry.rarity.is_supplied());
    assert_eq!(entry.rarity.supplied(), None);
    assert_eq!(entry.rarity, OfferedAttribute::Undisclosed);
    assert_eq!(entry.name.supplied(), Some("Strike"));
    assert_eq!(entry.description.supplied(), Some("deal 6 damage"));

    // An absent attribute defaults to absence, never to a value.
    let defaulted: OfferedAttribute<&str> = OfferedAttribute::default();
    assert!(defaulted.is_undisclosed());
    assert_eq!(defaulted.supplied(), None);
    assert_eq!(OfferedAttribute::from(None::<&str>), defaulted);
    assert_eq!(
        OfferedAttribute::from(Some("common")),
        OfferedAttribute::Supplied("common")
    );

    // The disclosed-attribute set names only what the host supplied.
    assert_eq!(
        entry.disclosed_attributes(),
        &[
            OfferedAttributeName::Name,
            OfferedAttributeName::Description,
        ]
    );
    assert!(
        !entry
            .disclosed_attributes()
            .contains(&OfferedAttributeName::Rarity)
    );

    // An undisclosed attribute carries no value, so it cannot exceed a length bound or fail one.
    assert_eq!(validate_offered_entry(&entry), Ok(()));
}

#[test]
fn rarity_is_only_the_hosts_own_spelling_and_carries_no_unit_of_weight() {
    let set = [
        fully_described("card.definition", "common"),
        fully_described("synthetic.card.rare", "the boss's own odd spelling"),
    ];
    assert_eq!(validate_offered_set(&set), Ok(()));
    assert_eq!(set[0].rarity.supplied(), Some("common"));
    assert_eq!(
        set[1].rarity.supplied(),
        Some("the boss's own odd spelling")
    );
    assert_ne!(set[0].rarity.supplied(), set[1].rarity.supplied());

    // Every modelled attribute is dimensionless text: no numeric or probability-bearing unit is
    // reachable for a rarity, so no weight can be read out of this type.
    for entry in &set {
        for (attribute, unit) in entry.attribute_units() {
            assert_eq!(unit, RuleUnit::Dimensionless, "{attribute:?}");
            assert_ne!(unit, RuleUnit::Multiplier);
            assert_ne!(unit, RuleUnit::Ratio);
        }
    }
    assert_eq!(
        OfferedAttributeName::Rarity.rule_unit(),
        RuleUnit::Dimensionless
    );
}

#[test]
fn empty_identities_and_oversized_attributes_are_rejected_without_defaults() {
    let empty = offered_entry(
        OfferedEntryKind::Card,
        "",
        OfferedAttribute::Supplied("Strike"),
        OfferedAttribute::Supplied("common"),
        OfferedAttribute::Undisclosed,
    );
    assert_eq!(
        validate_offered_entry(&empty),
        Err(OfferedSetError::EmptyIdentity {
            kind: OfferedEntryKind::Card
        })
    );

    let long = "x".repeat(MAX_OFFERED_ATTRIBUTE_BYTES + 1);
    let oversized = offered_entry(
        OfferedEntryKind::Card,
        "card.definition",
        OfferedAttribute::Supplied(long.as_str()),
        OfferedAttribute::Supplied("common"),
        OfferedAttribute::Undisclosed,
    );
    assert_eq!(
        validate_offered_entry(&oversized),
        Err(OfferedSetError::AttributeTooLong {
            kind: OfferedEntryKind::Card,
            attribute: OfferedAttributeName::Name,
        })
    );

    // The bound is inclusive: the longest accepted attribute still validates.
    let at_bound = "x".repeat(MAX_OFFERED_ATTRIBUTE_BYTES);
    let boundary = offered_entry(
        OfferedEntryKind::Card,
        "card.definition",
        OfferedAttribute::Supplied(at_bound.as_str()),
        OfferedAttribute::Undisclosed,
        OfferedAttribute::Undisclosed,
    );
    assert_eq!(validate_offered_entry(&boundary), Ok(()));
}

#[test]
fn the_acquisition_exclusion_is_narrowed_to_generation_policy_only() {
    let exclusions = unmodeled_combinations(RuleFamily::Acquisition);
    assert_eq!(exclusions.len(), 1);
    let exclusion = exclusions[0];

    // Retained, and narrowed to exactly the generation-policy half.
    assert_eq!(exclusion.family, RuleFamily::Acquisition);
    assert_eq!(exclusion.rules, &[RuleId::RewardChoicePicks]);
    for excluded in ["generation weights", "rerolls", "undisclosed outcomes"] {
        assert!(
            exclusion.combination.contains(excluded) || exclusion.reason.contains(excluded),
            "narrowed exclusion no longer names {excluded}: {exclusion:?}"
        );
    }

    // The stale "unmodeled, so no identity or rarity is inferred" reason is gone, and the reason
    // now says identity and attributes are modeled while generation policy stays excluded.
    assert!(!exclusion.reason.contains("the offered set is unmodeled"));
    assert!(exclusion.reason.contains("identity"));
    assert!(exclusion.reason.contains("modeled"));
    assert!(exclusion.reason.contains("generation weights"));

    // The coverage row's wording matches what is now modeled.
    let row = coverage_for(RuleFamily::Acquisition);
    assert_eq!(row.status, sts2_game_core::RuleCoverageStatus::Partial);
    assert!(row.unmodeled.contains("generation weights"));
    assert!(row.unmodeled.contains("rerolls"));
    assert!(!row.unmodeled.contains("rarity weighting"));

    // The record's own unmodeled list excludes generation policy and undisclosed entries, and no
    // longer claims card identity is unmodeled.
    let record = unmodeled_for(RuleId::RewardChoicePicks).unwrap_or_default();
    assert!(
        record
            .iter()
            .any(|entry| entry.contains("generation weights"))
    );
    assert!(record.iter().any(|entry| entry.contains("undisclosed")));
    assert!(
        !record
            .iter()
            .any(|entry| entry.contains("card identity and deck placement"))
    );
}
