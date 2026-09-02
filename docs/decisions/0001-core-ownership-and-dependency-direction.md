# ADR 0001: Core Ownership and Dependency Direction

- Status: Accepted for the foundation and future core implementation
- Date: 2026-09-02

## Context

The STS2 build has separate targets for semantic domain policy, host integration, lifecycle/routing,
MCP adaptation, experiment coordination, and an accepted sixth shared-contract target. The core
boundary must remain usable without a game process and must not become an accidental common crate for
side effects or sibling control-plane behavior.

## Decision

`sts2-game-core` owns only host-independent state, action descriptions, identity and generation
values, validation, typed domain errors, and deterministic policy. It may consume an explicitly
accepted neutral protocol contract, but it does not own HTTP, MCP, gateway, process, filesystem,
clock, provider, loader, FFI, or concrete-host behavior.

The intended compile-time edge is `sts2-game-mod -> sts2-game-core`. The gateway, MCP server, and
harness may use the accepted protocol target for their own neutral contracts; they do not reach into
core internals. The runtime path remains `harness -> MCP -> gateway -> isolated mod -> host`. The mod
retains authority to read host state and perform host mutations at its host boundary.

## Alternatives considered

- **Put all shared types in core:** rejected because it would make core an omnibus transport and
  lifecycle dependency and blur the host-independent boundary.
- **Let core call the host through callbacks:** rejected because authority, thread affinity, and
  unsafe lifetime rules belong to the mod boundary.
- **Duplicate semantic policy in each consumer:** rejected because it permits divergent validation;
  consumers should use the owned core or approved neutral contract instead.

## Consequences and evidence

Pure semantic tests can run on any supported developer/CI host. Host compatibility, transport
compatibility, process isolation, provider behavior, and release packaging remain separate gates and
are unverified by this initialization. The `crates/core` package now provides the pure semantic seam
without a protocol path dependency. A future dependency or boundary change needs a superseding ADR,
consumer list, compatibility classification, and deterministic conformance evidence.
