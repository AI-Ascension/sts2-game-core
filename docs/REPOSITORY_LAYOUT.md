# Repository Layout

## Current foundation tree

```text
sts2-game-core/
├── .github/                    # bounded read-only CI and dependency automation
├── crates/core/                # initialized pure semantic product crate and deterministic tests
├── protocol-artifact/poc-v1/   # copied release-like protocol data, not implementation code
├── schemas/domain/             # reserved owner-local semantic fixtures/shapes
├── conformance/                # reserved project-owned contract cases
├── docs/                       # target policy, architecture, and decisions
├── tests/                      # reserved cross-module deterministic tests
├── tools/repo-policy/          # target-local Rust governance checker
├── Cargo.toml                  # workspace for core and the governance tool
└── Cargo.lock                  # locked workspace dependencies
```

The root schemas, conformance, and tests directories remain responsibility markers. A future crate or
module may be added only with a named responsibility, consumer, build purpose, and non-empty test seam.

## Ownership and dependency direction

```text
sts2-protocol  --(accepted neutral contracts only)--> sts2-game-core
sts2-game-core -------------------------------------> sts2-game-mod

runtime: harness -> mcp-server -> gateway -> game-mod -> game host
```

Compile-time runtime/control-plane edges are intentionally not implied by the runtime arrows. Core
does not depend on host, transport, gateway, process, filesystem, MCP, provider, or model code.
`sts2-protocol` is the accepted sixth build-completion target, but receives only contracts with named
consumers, independent versioning, language/transport neutrality, provenance, and conformance.

## Provenance and generated content

This target is authored using the planning/project-policy standards and its own local repository
structure. Its only cross-target material is the inert copied `poc-v1` artifact, with source and
digest recorded in its manifest. It contains no copied product source, game assembly, save, credential,
or generated build output. Future generated schemas or fixtures must
record their source, generator, hash, license, and exact-path policy exception when necessary.

## Naming authority

The aggregate NAMING_CONVENTIONS.md is the shared naming authority, with the owner and exception
details in naming-registry.yaml. The concise
physical directory `crates/core` is intentionally mapped to the `sts2-game-core` package; it does
not change ownership.
