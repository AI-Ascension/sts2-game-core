# Security Policy

## Reporting

Report suspected vulnerabilities or accidental exposure of private data through the project's private
maintainer/security channel. Do not publish credentials, personal saves, proprietary host assemblies,
or exploit details in a public issue. If no private channel is configured yet, contact the repository
maintainers privately and include only a sanitized reproduction.

## Scope

Core is intended to be a pure, host-independent library. Security-sensitive concerns include malformed
state/action input, stale generations, ambiguous identity, validation bypasses, nondeterministic ordering,
unbounded values, information leakage through errors, and accidental dependency on a side-effecting
boundary.

Transport authentication, listener exposure, process isolation, host thread affinity, loader/FFI safety,
profiles, saves, and provider credentials belong to the mod, gateway, MCP, or harness boundary. Do not
solve them by importing those concerns into core.

## Handling

Maintainers will acknowledge reports, reproduce them with sanitized and disposable fixtures, coordinate
a fix or mitigation, and publish only the minimum necessary detail. Security corrections must include a
regression test, compatibility classification, and release note. Never use a valued profile or real
credentials for testing.
