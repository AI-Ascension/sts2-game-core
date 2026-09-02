# Development and Automation Workflows

## Change lifecycle

1. Identify the core owner, consumer, contract, and evidence needed.
2. Read [`AGENTS.md`](../AGENTS.md) and the relevant target docs.
3. Record a decision for ownership, dependency, public-contract, or protocol-scope changes.
4. Add deterministic requirements/tests before or with product behavior.
5. Run strict policy, metadata, format, Clippy, and tests; report unavailable gates explicitly.
6. Review provenance, security, compatibility, changelog, and claims with unverified runtime evidence.

Keep pull requests focused. Do not combine core semantic work with host loader, gateway lifecycle,
MCP, harness/provider, deployment, or release work.

## Automation topology

`policy.yml` runs the target-local policy tool. `ci.yml` runs Rust formatting, Clippy, and tests for
the current governance workspace and initialized core package.
Both workflows use `pull_request` and pushes to `main`, explicit top-level `contents: read`, bounded
timeouts, cancellation for superseded pull requests, and an immutable checkout action.

Workflows do not use secrets, trusted self-hosted networks, proprietary host files, valued profiles,
providers, arbitrary refs, `pull_request_target`, `continue-on-error: true`, or unconditional success
commands. Any release or host-compatible lane must be separately approved and protected.

## Pull requests and branch protection

The pull-request template requires outcome, boundary, compatibility, exact evidence, skipped checks,
and remaining risks. Once hosted, repository settings should require stable policy and CI checks;
workflow YAML alone cannot prevent an administrator from bypassing branch protection.

## Release and dependency changes

Dependency changes need a lockfile, license review, security review, and notice impact. Toolchain
changes update `rust-toolchain.toml`, CI, compatibility notes, and release evidence together. Releases
follow [`../RELEASING.md`](../RELEASING.md) and require explicit maintainer authorization.
