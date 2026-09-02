# Licensing Policy

## Decision

Original core code and documentation are licensed under MIT. Contributors must have the right to
submit their work under that license. This policy is engineering guidance, not legal advice.

## Boundaries

MIT covers only material the project can license. It does not grant rights to STS2 game binaries,
game data, art, music, trademarks, platform components, host assemblies, user saves, or external
installations. Those materials must remain outside this target and outside release archives.

The target-local policy tool uses Cargo dependencies to parse its policy configuration. Their exact
versions are locked in [`../Cargo.lock`](../Cargo.lock); dependency notices and license metadata must
be regenerated and reviewed from that exact lockfile before distribution.

## Greenfield and provenance rules

- Write original product code and fixtures, or record reviewed redistribution rights for imported data.
- Do not copy, vendor, transliterate, or use another harness's source as an implementation plan.
- Put `SPDX-License-Identifier: MIT` in every Rust source file.
- Preserve notices for adapted or generated material and document its generator and source.
- Block a release when a dependency, fixture, or host artifact has unknown or incompatible licensing.

See [`LICENSE`](../LICENSE) and [`THIRD_PARTY_NOTICES.md`](../THIRD_PARTY_NOTICES.md).
