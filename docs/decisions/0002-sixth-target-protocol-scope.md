# ADR 0002: Accepted Sixth-Target Protocol Scope

- Status: Accepted for the current build-completion run
- Date: 2026-09-02

## Context

Earlier planning material treated `sts2-protocol` as a decision-stage candidate and recommended
deferral. The current build-completion direction explicitly accepts it as the sixth implementation
target, while requiring it to remain narrow rather than becoming a generic ownership escape hatch.
This core target must reflect that current decision without inventing a protocol dependency or moving
core-specific semantics out of their owner. The core package is initialized separately and does not
create a protocol dependency.

## Decision

`sts2-protocol` is an accepted sixth target for the run. It may own a contract only when the contract
has at least two named consumers, a named owner, an independent version/compatibility policy,
language-neutral and transport-neutral representation, recorded provenance, and implementation-neutral
conformance fixtures. The protocol target must not own game rules, host objects, gateway lifecycle,
MCP catalogs, model/provider behavior, process control, or boundary-specific routes.

Core remains the owner of semantic state/action/identity/validation policy. Core may depend on a
protocol contract only after that contract passes the gates above and a separate dependency review
records the exact package/version and compatibility impact. This foundation wave adds no protocol
crate, schema, or path dependency.

## Alternatives considered

- **Retain the old defer disposition:** rejected for this run because the current build-completion
  decision explicitly includes the sixth target.
- **Move every cross-target type to protocol:** rejected because shared transport or host details do
  not become neutral merely by being placed in a sixth repository.
- **Create a core-to-protocol dependency now:** deferred because no accepted contract or named second
consumer is implemented in this core initialization wave.

## Consequences and evidence

The six-target topology is recorded while core remains pure and independently testable. The accepted
status of the sixth target does not establish a released protocol, serialization compatibility, or
runtime behavior. Those claims remain `unverified` until the protocol owner records its contracts,
fixtures, versioning, consumers, and passing conformance evidence.
