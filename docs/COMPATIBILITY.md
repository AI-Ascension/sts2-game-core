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
| Core domain contract | Unreleased package v0.0.0; deterministic `use_budget`, frozen `end_turn`, and checked Runtime-v2 projection seams | Confirmed by deterministic unit/integration tests; not a frozen game contract |
| POC artifact mapping | `sts2-protocol/poc-v1` and its recorded schema digest | Confirmed local manifest/schema/fixture presence; no protocol implementation or runtime claim |
| Game host/runtime | Not owned by this target | Unverified here; mod-owned evidence required |

The initial and Runtime-v2 seams are internal semantic contracts for this target, not frozen wire or
game-mechanics compatibility promises. A successful policy, metadata, format, lint, or test run does
not establish host/runtime compatibility. Runtime-v2 operation identity, receipt, reconciliation,
and idempotency behavior remains a later boundary contract.
The checked Runtime-v2 projection rejects turn indices above 1024 and generations above
`9_007_199_254_740_991`; it is a representational guard, not a protocol serialization or host
compatibility guarantee.

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
be reused by the mod without inheriting its host claims. The sixth protocol target owns the shared
artifact representation; this target verifies copied bytes as data and does not infer semantic or
runtime compatibility from that check.

## Evidence language

Use `confirmed` for reproduced target evidence, `source-derived` for observations from authorized
source/material, `proposed` for design, `inferred` for reasoned but untested conclusions, and
`unverified` when a runtime or external precondition has not been executed. Never promote a static
document, mock, or compile pass into a host/runtime claim.

## Complete POC artifact inventory

The POC package and mirrored source schema/conformance case are copied byte-for-byte from
`AI-Ascension/sts2-protocol` main commit `11e4252e39a77f0017b8e4f3720590e6162e8f53`.
The original eight consumer payloads remain unchanged. The canonical README and ten-entry
`SHA256SUMS` replace the earlier consumer-specific packaging; neither is regenerated locally.
The added root mirrors satisfy the inventory's relative paths and do not transfer schema or
conformance authority to core. This is a packaging safety correction with no API or wire change.
Byte checks establish artifact identity only; runtime and protocol-implementation parity remain
unverified. Core still intentionally consumes no Runtime-v1/v2/v3 serialization artifact.
