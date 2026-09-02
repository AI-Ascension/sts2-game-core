// SPDX-License-Identifier: MIT

use serde_json::Value;

/// Version consumed by this core POC mapping.
pub const POC_PROTOCOL_VERSION: &str = "poc-v1";
/// Schema digest supplied by the protocol release-like artifact.
pub const POC_SCHEMA_DIGEST: &str =
    "adb434d119a51b00d968e71bf0bf774f2a08de7c875a5479900aa34b3c02e027";
/// Release-like artifact identity, not a Rust package dependency.
pub const POC_ARTIFACT: &str = "sts2-protocol/poc-v1";

const MANIFEST: &str = include_str!("../../../protocol-artifact/poc-v1/manifest.json");
const SCHEMA: &str = include_str!("../../../protocol-artifact/poc-v1/schema.json");
const GOLDEN: &str = include_str!("../../../protocol-artifact/poc-v1/golden/action-accepted.json");
const INVALID: &str =
    include_str!("../../../protocol-artifact/poc-v1/fixtures/invalid-action.json");

/// Verifies the local copy of the protocol artifact before a POC mapping uses it.
///
/// # Errors
///
/// Returns an [`ArtifactError`] when a copied manifest, schema, or fixture is malformed or does not
/// identify the expected release-like artifact.
pub fn verify_poc_artifact() -> Result<(), ArtifactError> {
    let manifest = parse(MANIFEST)?;
    if manifest["artifact"] != POC_ARTIFACT
        || manifest["protocol_version"] != POC_PROTOCOL_VERSION
        || manifest["schema_digest"] != POC_SCHEMA_DIGEST
    {
        return Err(ArtifactError::ManifestMismatch);
    }
    if parse(SCHEMA)?["$id"] != "sts2-poc-v1" {
        return Err(ArtifactError::SchemaMismatch);
    }
    parse(GOLDEN)?;
    parse(INVALID)?;
    Ok(())
}

/// A deterministic failure while loading the checked-in release-like artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactError {
    InvalidJson,
    ManifestMismatch,
    SchemaMismatch,
}

fn parse(text: &str) -> Result<Value, ArtifactError> {
    serde_json::from_str(text).map_err(|_| ArtifactError::InvalidJson)
}
