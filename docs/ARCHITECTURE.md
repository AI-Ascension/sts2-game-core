# Core Architecture

## Purpose and owner

`sts2-game-core` owns the host-independent semantic layer: domain state, action descriptions,
identity, generation values, validation, typed domain errors, and deterministic policy. The core
maintainers own these concepts and their project-defined contracts. This document is the boundary
authority for this target; public behavior also needs a requirement and deterministic test.

## System ownership

The six-target system has deliberately separate responsibilities:

| Boundary | Owns | Must not be moved into core |
|---|---|---|
| `sts2-game-core` | semantic state/action policy and typed values | transport, host, process, persistence, lifecycle |
| `sts2-game-mod` | host authority, loader/FFI, main-thread translation, host-facing adapter | MCP, gateway lifecycle, portable host policy |
| `sts2-gateway` | instance lifecycle, leases, fencing, routing, isolation, health | game rules, host objects, MCP semantics |
| `sts2-mcp-server` | MCP framing, catalog, mapping, bounded adapter behavior | host access, game rules, gateway registry |
| `sts2-harness` | coordination, experiments, providers, trajectories, artifacts | game process control, host authority, MCP server behavior |
| `sts2-protocol` | only accepted shared language/transport-neutral contracts | an omnibus common crate or boundary-specific behavior |

Runtime communication and compile-time dependency edges are different:

```text
Runtime:       harness -> MCP server -> gateway -> isolated game mod -> game host
Compile time:   game mod -> core
               core/gateway/MCP/harness -> released protocol artifacts only
```

Core has no edge to HTTP, MCP, gateway, process, filesystem, concrete host, loader, or model code.
The future mod may depend on core; sibling control-plane targets do not reach through core to obtain
host or game behavior.

## Domain seams

The initialized core package is split into cohesive modules:

- **State** represents owned snapshots, scoped values, and explicit point-in-time semantics.
- **Actions** represents typed proposals and their declared arguments, not host calls.
- **Identity** represents actor/session and generation scope without inferring identity from position.
- **Validation** checks a proposal against a supplied snapshot and returns structured results.
- **Policy** defines deterministic ordering and failure precedence where the core contract requires it.

The current package is `sts2-game-core` at `crates/core`. It consumes only the checked-in
`protocol-artifact/poc-v1` data and has no path dependency on `sts2-protocol` or another product
implementation. The root Cargo workspace also contains the independent governance checker.

Validation can reject stale, malformed, or illegal input. It cannot authorize or perform a host
mutation, and acceptance by core never means that a host transition completed. The mod re-reads host
state and retains authority at its boundary.

## Dependency rules

Dependencies point inward toward stable semantic abstractions. Product core code may use the Rust
standard library and an explicitly approved neutral serialization/value dependency, but must not add
transport, process, filesystem, clock, concrete-host, loader, provider, or sibling application
dependencies. The local artifact verifier requires the POC version, schema digest, manifest identity,
schema ID, and JSON fixture presence; it is not conformance with the protocol Rust implementation.

No unsafe code is permitted in this target. Unsafe host/FFI work belongs to the mod boundary, where
its lifetime, thread, ownership, and unload invariants can be reviewed separately.

## Change control

Changes to ownership, dependency direction, public shape, validation order, identity lifetime, or
protocol relationships require a decision record under `docs/decisions/`. Do not create a product
crate merely to fill the reserved scaffold. See [`PRODUCT.md`](PRODUCT.md),
[`COMPATIBILITY.md`](COMPATIBILITY.md), and [`TESTING.md`](TESTING.md).
