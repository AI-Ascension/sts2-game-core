// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

use crate::diagnostic::Finding;
use crate::files::relative_text;

const CORE_PREFIX: &str = "crates/core/";
const FORBIDDEN_TERMS: &[&str] = &[
    "std::fs",
    "std::net",
    "std::path",
    "std::process",
    "systemtime",
    "reqwest",
    "hyper",
    "axum",
    "tokio",
    "mcp",
    "gateway",
    "godot",
    "megacrit",
    "modinitializer",
];

pub(crate) fn findings(root: &std::path::Path, files: &[PathBuf]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for path in files {
        let relative = relative_text(root, path);
        if !relative.starts_with(CORE_PREFIX)
            || path.extension().and_then(|value| value.to_str()) != Some("rs")
        {
            continue;
        }
        let Ok(text) = fs::read_to_string(path) else {
            continue;
        };
        let lower = text.to_ascii_lowercase();
        for term in FORBIDDEN_TERMS {
            if lower.contains(term) {
                findings.push(Finding::error(
                    "BOUND001",
                    &relative,
                    format!("core source contains forbidden boundary term `{term}`"),
                ));
            }
        }
    }
    findings
}
