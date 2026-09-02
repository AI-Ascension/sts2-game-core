# ADR 0003: Minimal POC Semantic Slice

- Status: Accepted for the deterministic fake vertical slice
- Date: 2026-09-02

## Decision

`sts2-game-core` remains the sole owner of semantic legality and state transition meaning. It exposes
one deterministic `use_budget` action with a typed `units` argument, rejects zero units, and applies
one accepted action by advancing generation, reducing available units, and incrementing a settled
effect witness exactly once.

The core verifies the copied `sts2-protocol/poc-v1` manifest, schema ID, digest, and fixture presence
as inert data. It does not import protocol implementation internals or depend on a sibling repository.
The game-mod boundary supplies host/thread translation and settlement evidence through its own seam.

## Evidence and limits

Offline tests prove the state transition, invalid-action no-op, stale-generation rejection, and local
artifact metadata. They do not prove a game host, loader, runtime transport, or cross-repository
binary compatibility.
