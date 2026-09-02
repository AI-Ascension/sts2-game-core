# Changelog

All notable user-visible changes to this target will be documented here. The project follows
Semantic Versioning once versioned releases begin.

## Unreleased

### Added

- Target-local repository governance and contributor guidance.
- Core-specific architecture, product, compatibility, testing, licensing, workflow, and policy docs.
- A Rust-only governance workspace with a strict repository-policy checker.
- A non-empty `sts2-game-core` package with typed identity, generation, state, action, and pure
  validation seams plus deterministic valid/invalid/stale/reproducibility tests.
- Ownership and dependency decisions for the host-independent core boundary and accepted sixth target.
- Deterministic `use_budget` application, settled-effect witness state, and verification of the copied
  release-like `sts2-protocol/poc-v1` artifact.

### Changed

- The semantic seam now demonstrates one checked state transition while preserving the host-free
  boundary; no protocol implementation path dependency was added.

### Deprecated

- Nothing yet.

### Removed

- Nothing yet.

### Fixed

- Nothing yet.

### Security

- Nothing yet.
