# Core Compatibility Policy

## Scope

Compatibility for this target is semantic contract compatibility: typed state/action values,
identity and generation scope, validation results, error categories, ordering, and any later
project-owned serialization fixtures. Core does not claim compatibility with a game host, loader,
HTTP route, MCP tool, gateway process, provider, save format, or another harness.

## Current evidence baseline

| Subject | Version or identity | Evidence level |
|---|---|---|
| Repository foundation | Unreleased target preparation | Confirmed static structure only |
| Core domain contract | Unreleased package v0.0.0; initial semantic seam | Confirmed by deterministic unit/integration tests; not a frozen wire contract |
| Game host/runtime | Not owned by this target | Unverified here; mod-owned evidence required |

The initial seam is an internal semantic contract for this target, not a frozen wire or game-mechanics
compatibility promise. A successful policy, metadata, format, lint, or test run does not establish
host/runtime compatibility.

## Compatibility classes

Classify every future change as one of:

- **Internal:** no observable contract or consumer change.
- **Additive-compatible:** an optional value or capability with unchanged valid behavior.
- **Safety correction:** a documented invalid or unsafe case is rejected more safely, with migration
  and release notes.
- **Deprecated-compatible:** old behavior remains during a documented replacement window.
- **Breaking:** meaning, required data, validation order, identity lifetime, or error contract changes.

Each non-internal change needs a requirement/test identifier, affected consumers, fixture updates,
version impact, migration notes, and explicit unverified evidence. Do not infer core compatibility
from a mod build or a protocol package version.

## Host and protocol separation

The mod owns exact host-version, loader, ABI, main-thread, and disposable-profile evidence. Core may
be reused by the mod without inheriting its host claims. The sixth protocol target owns only shared
contracts that pass its consumer, neutrality, provenance, versioning, and conformance gates. A
protocol dependency is not introduced merely because two targets exchange data at runtime.

## Evidence language

Use `confirmed` for reproduced target evidence, `source-derived` for observations from authorized
source/material, `proposed` for design, `inferred` for reasoned but untested conclusions, and
`unverified` when a runtime or external precondition has not been executed. Never promote a static
document, mock, or compile pass into a host/runtime claim.
