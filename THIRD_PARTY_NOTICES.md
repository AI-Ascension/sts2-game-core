# Third-Party Notices

This foundation target contains no copied product implementation, proprietary game material, host
assembly, save, or generated release artifact.

The target-local governance checker uses the Rust `toml` crate and its locked transitive dependencies
only to parse `policy.toml`; they are not part of the future game-domain API. Their exact versions are
recorded in [`Cargo.lock`](Cargo.lock). Before distributing any executable or source bundle, generate a
notice from the exact lockfile and resolved registry metadata, verify every license, and retain all
applicable notices. An unknown or incompatible dependency license blocks release.

The core artifact verifier also uses the Rust `sha2` crate to verify the checked-in SHA-256 inventory;
its exact version and transitive dependencies are recorded in [`Cargo.lock`](Cargo.lock).

Project-authored code and documentation are under [`LICENSE`](LICENSE). That license does not grant
rights to STS2 game binaries, data, art, music, trademarks, platform components, or external host
installations.
