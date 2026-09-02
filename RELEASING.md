# Release Policy and Procedure

Releases are deliberate, immutable, evidence-backed publications. Preparing a build, publishing it,
and verifying the published bytes are separate states.

## Current status

This target has no release version. The initialized workspace contains one pure core library and the
target-local Rust governance tool; no packaged product artifact is defined. Do not package empty
responsibility directories, proprietary game files, local saves, credentials, or generated build output.

## Required gates before a future release

- approved target and product version with matching changelog and compatibility records;
- clean, reviewed source at an immutable revision and a committed lockfile;
- strict repository policy, formatting, Clippy, unit, schema, and conformance checks;
- exact contract fixtures and a reviewed dependency/license notice;
- proof that the core dependency graph contains no transport, process, filesystem, or concrete-host
  implementation; and
- a coordinated consumer review for any public state, action, identity, validation, error, or protocol
  change.

Core releases do not establish game-host compatibility. Host load/runtime and packaged mod evidence are
owned by `sts2-game-mod` and must be reported separately. A core build or test pass is not a host smoke
test, end-to-end result, merge approval, or publication authorization.

## Publication controls

Only an explicitly authorized maintainer may tag, publish, or deploy. Release automation must use the
approved commit, bounded allowlists, protected environments, and exact checksums. Do not rebuild different
bytes during promotion. If a contract changes, classify it as internal, additive-compatible, safety
correction, deprecated-compatible, or breaking and document migration before publication.
