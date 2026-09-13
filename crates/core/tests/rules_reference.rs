// SPDX-License-Identifier: MIT

use sts2_game_core::{EvidenceStatus, RuleFamily, RuleId, RuleLookup, RuleSupport, rules_for};

#[test]
fn documented_calculators_are_never_promoted_to_host_parity() {
    let RuleLookup::Found(rule) = rules_for(RuleId::NominalCardDamage) else {
        panic!("missing rule")
    };
    assert_eq!(rule.evidence, EvidenceStatus::SimplifiedModel);
    assert_eq!(rule.support, RuleSupport::Conditional);
    assert_eq!(rule.steps[0].order, 1);
    assert_eq!(
        rule.steps[0].description,
        "multiply visible card damage by visible hit count for one target"
    );
    assert!(rule.assumptions.contains("per-target damage only"));
}

#[test]
fn unmodeled_families_are_explicitly_unsupported() {
    assert!(matches!(
        RuleFamily::Heal.lookup(),
        RuleLookup::Unsupported { .. }
    ));
    assert!(matches!(
        RuleFamily::EntityInteraction.lookup(),
        RuleLookup::Unsupported { .. }
    ));
}
