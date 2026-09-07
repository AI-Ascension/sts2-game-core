// SPDX-License-Identifier: MIT

use super::{Policy, SizeCategory};

const LIMITS: &str = r"
[limits]
rust_production_preferred = 10
rust_production_max = 20
rust_test_preferred = 11
rust_test_max = 21
csharp_production_preferred = 12
csharp_production_max = 22
csharp_test_preferred = 13
csharp_test_max = 23
workflow_preferred = 14
workflow_max = 24
markdown_preferred = 15
markdown_max = 25
";

#[test]
fn parses_complete_policy() -> Result<(), String> {
    let text = format!(
        "policy_version = 1\n[project]\nrequired_files = [\"README.md\"]\n\
         ignored_directories = [\"target\"]\nignored_path_prefixes = []\n\
         {LIMITS}\n[exemptions]\n"
    );
    let policy = Policy::parse(&text)?;
    assert_eq!(policy.required_files, ["README.md"]);
    assert_eq!(policy.budget(SizeCategory::RustProduction).maximum, 20);
    Ok(())
}

#[test]
fn rejects_inverted_budget() {
    let text = format!(
        "policy_version = 1\n[project]\nrequired_files = []\n\
         ignored_directories = []\nignored_path_prefixes = []\n\
         {}\n[exemptions]\n",
        LIMITS.replace("rust_production_max = 20", "rust_production_max = 5")
    );
    assert!(Policy::parse(&text).is_err());
}

#[test]
fn rejects_unknown_and_duplicate_severity_rules() {
    let base = format!(
        "policy_version = 2\n[project]\nrequired_files = []\nignored_directories = []\nignored_path_prefixes = []\n[severity]\nmandatory = [\"*\"]\nadvisory = [\"SIZE001\"]\n{LIMITS}\n[exemptions]\n"
    );
    assert!(
        Policy::parse(&base.replace("advisory = [\"SIZE001\"]", "advisory = [\"RUST999\"]"))
            .is_err()
    );
    assert!(
        Policy::parse(&base.replace("advisory = [\"SIZE001\"]", "advisory = [\"RUST001\"]"))
            .is_err()
    );
    assert!(
        Policy::parse(&base.replace("mandatory = [\"*\"]", "mandatory = [\"*\", \"*\"]")).is_err()
    );
    assert!(
        Policy::parse(&base.replace(
            "advisory = [\"SIZE001\"]",
            "advisory = [\"SIZE001\", \"SIZE001\"]"
        ))
        .is_err()
    );
}

#[test]
fn rejects_broad_standards_ignore_but_accepts_exact_managed_path() {
    let base = format!(
        "policy_version = 2\n[project]\nrequired_files = []\nignored_directories = []\nignored_path_prefixes = [\"standards/tools/standards-sync\"]\n[severity]\nmandatory = [\"*\"]\nadvisory = [\"SIZE001\"]\n{LIMITS}\n[exemptions]\n"
    );
    assert!(Policy::parse(&base).is_ok());
    assert!(Policy::parse(&base.replace("standards/tools/standards-sync", "standards")).is_err());
}

#[test]
fn rejects_path_traversal_in_required_files() {
    let text = format!(
        "policy_version = 1\n[project]\nrequired_files = [\"../outside\"]\n\
         ignored_directories = []\nignored_path_prefixes = []\n\
         {LIMITS}\n[exemptions]\n"
    );
    assert!(Policy::parse(&text).is_err());
}

#[test]
fn rejects_duplicate_paths_and_broad_ignored_directories() {
    let base = format!(
        "policy_version = 1\n[project]\nrequired_files = [\"README.md\"]\n\
         ignored_directories = [\"target\"]\nignored_path_prefixes = []\n\
         {LIMITS}\n[exemptions]\n"
    );
    assert!(
        Policy::parse(&base.replace(
            "required_files = [\"README.md\"]",
            "required_files = [\"README.md\", \"README.md\"]"
        ))
        .is_err()
    );
    assert!(
        Policy::parse(&base.replace(
            "ignored_directories = [\"target\"]",
            "ignored_directories = [\"target\", \"target\"]"
        ))
        .is_err()
    );
    assert!(
        Policy::parse(&base.replace(
            "ignored_directories = [\"target\"]",
            "ignored_directories = [\"standards\"]"
        ))
        .is_err()
    );
}
