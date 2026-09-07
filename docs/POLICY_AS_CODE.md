# Policy as Code

## Purpose and entrypoint

Written instructions are advisory until measurable parts are checked. This target keeps objective
foundation rules in [`../policy.toml`](../policy.toml) and checks them with the target-local Rust
`repo-policy` tool. From the target root, the canonical strict command is:

```bash
cargo run --locked --package repo-policy -- --strict
```

The command is read-only over the repository tree and returns nonzero for mandatory failures. The
governance tool is not product behavior and has no dependency on core or any sibling target.

## Rule families

| Rule | Enforcement |
|---|---|
| `CFG001` | Policy exists, parses, and uses the supported version |
| `DOC001` | Required foundation files exist |
| `DOC002` | Local Markdown links resolve |
| `SIZE001` | Rust, C#, workflow, and Markdown budgets are respected |
| `EXC001` | Exemptions are exact existing paths with meaningful reasons |
| `LANG001` | Python source and package metadata are rejected |
| `WF001-005` | Explicit permissions, no trust escalation/suppression, immutable actions |
| `RUST001` | Lockfile, toolchain, workspace metadata, and inherited lints are present |
| `LIC001-003` | MIT root/manifest declarations and source SPDX headers are present |
| `BOUND001` | Future `crates/core` source cannot import known side-effecting boundaries |

Policy version 2 classifies findings explicitly: the default is mandatory and `SIZE001` preferred-size
guidance is advisory. Strict mode fails mandatory findings and hard size limits while retaining
advisory diagnostics. Policy version 1 remains available for legacy callers; its strict mode promotes
every warning, so migration requires changing the version and adding the severity table deliberately.
Exemptions are exact paths and require provenance/reason. Copied reference implementations are never
eligible for exemption.

## CI and limits

The policy workflow runs the same locked tool and has only `contents: read`, bounded execution time,
and an immutable checkout action. The CI workflow runs formatting, Clippy, and tests for the
governance workspace and the pure core package. No workflow accesses game files, saves, credentials,
providers, or sibling repositories.

Static policy cannot prove cohesive domain design, host behavior, runtime compatibility, or external
branch protection. Review and deterministic tests remain necessary when product source is introduced.

## Production lint scope

The production Clippy lane selects workspace libraries and binaries and forbids
unwrap, expect, panic, todo and unimplemented on the compiler command line.
A source-level allowance cannot override that lane. The existing all-target lane
still checks tests with their scoped allowances. `production_lints` runs real
compiler fixtures for forbidden constructs, an attempted blanket allowance,
and valid comments/test-only code. Missing Clippy or an unrelated compiler
failure cannot satisfy a negative case: its diagnostic must identify the rule.
