# Contributing

This target is the host-independent semantic foundation for the STS2 project. Read [`AGENTS.md`](AGENTS.md),
[`docs/PRODUCT.md`](docs/PRODUCT.md), [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md),
[`docs/CODING_STANDARDS.md`](docs/CODING_STANDARDS.md), and [`docs/TESTING.md`](docs/TESTING.md)
before changing it.

## Scope

Core owns typed state, action descriptions, identity and generation values, validation, deterministic
ordering, and domain-level errors. It does not own HTTP, MCP, gateway lifecycle, processes, filesystems,
clocks, concrete host objects, host authority, orchestration, providers, or artifacts. The game mod
remains authoritative for host state and mutations; core can validate a domain request but cannot claim
that a host operation completed.

This is an original greenfield target. Do not copy or transliterate another harness. The initialized
semantic seam is deliberately small and project-owned; define any extension in requirements,
schemas/fixtures where appropriate, and deterministic tests. Do not add transport, host, lifecycle,
provider, storage, or orchestration behavior here.

## Development workflow

1. Identify the owning boundary and affected contract.
2. Record a decision when ownership, dependency direction, or public behavior changes.
3. Add deterministic tests for accepted, rejected, stale, and invalid inputs as applicable.
4. Run the policy, format, Clippy, metadata, and test commands in [`docs/TESTING.md`](docs/TESTING.md).
5. Update compatibility, licensing, architecture, and changelog documentation when relevant.
6. Describe runtime-unverified evidence and checks not run in the pull request.

Discuss first when changing a public state/action shape, validation order, identity lifetime, error
meaning, serialization, dependency boundary, or protocol-consumer relationship. Never add a dependency
on a transport, host, process, filesystem, or sibling control-plane implementation to solve a core task.

## Contribution and release rules

Contributors must have the right to submit their work under the repository's [`MIT License`](LICENSE).
Retain notices for adapted or generated material. Do not submit proprietary game assets, host
assemblies, personal saves, credentials, or unknown-license fixtures. Maintainers handle releases under
[`RELEASING.md`](RELEASING.md); contribution does not authorize tagging, publishing, deployment, or merge.
