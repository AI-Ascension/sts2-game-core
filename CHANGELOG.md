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
- A typed, pure Runtime-v2 `end_turn` semantic action with session-scoped combat snapshots,
  deterministic phase/generation/bounds rejection, and a settled domain effect witness.
- A checked Runtime-v2 observation projection that rejects turn indices above 1024 and generations
  above the safe-integer maximum `9_007_199_254_740_991` with typed errors.

### Changed

- The semantic seam now demonstrates one checked state transition while preserving the host-free
  boundary; no protocol implementation path dependency was added.
- The Runtime-v2 combat seam is additive to the existing POC API and keeps operation idempotency,
  receipt storage, and host execution at later owning boundaries.
- Existing domain constructors remain unchanged; representational bounds are enforced only by the
  explicit Runtime-v2 projection.

### Deprecated

- Nothing yet.

### Removed

- Nothing yet.

### Fixed

- Preserve the complete protocol-owner POC artifact package and checksum inventory, including
  source-schema and conformance mirrors, while leaving the original eight payloads unchanged.

### Security

- Nothing yet.
