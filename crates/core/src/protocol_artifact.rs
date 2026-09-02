// SPDX-License-Identifier: MIT

use serde_json::Value;
use sha2::{Digest, Sha256};

/// Version consumed by this core POC mapping.
pub const POC_PROTOCOL_VERSION: &str = "poc-v1";
/// Schema digest supplied by the protocol release-like artifact.
pub const POC_SCHEMA_DIGEST: &str =
    "adb434d119a51b00d968e71bf0bf774f2a08de7c875a5479900aa34b3c02e027";
/// Release-like artifact identity, not a Rust package dependency.
pub const POC_ARTIFACT: &str = "sts2-protocol/poc-v1";

const POC_SCHEMA_SOURCE: &str = "schemas/poc-v1.schema.json";
const POC_GENERATOR: &str = "hand-authored";
const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
const MANIFEST: &str = include_str!("../../../protocol-artifact/poc-v1/manifest.json");
const CHECKSUMS: &str = include_str!("../../../protocol-artifact/poc-v1/SHA256SUMS");
const SCHEMA: &str = include_str!("../../../protocol-artifact/poc-v1/schema.json");
const STATE: &str = include_str!("../../../protocol-artifact/poc-v1/golden/state-response.json");
const GOLDEN: &str = include_str!("../../../protocol-artifact/poc-v1/golden/action-accepted.json");
const INVALID: &str =
    include_str!("../../../protocol-artifact/poc-v1/fixtures/invalid-action.json");

/// Verifies the local copy of the protocol artifact before a POC mapping uses it.
///
/// The checked-in inventory is matched to the expected release-like files and each consumed file is
/// hashed before its JSON metadata is inspected.
///
/// # Errors
///
/// Returns an [`ArtifactError`] when a copied manifest, schema, checksum inventory, or fixture is
/// malformed or does not identify the expected release-like artifact.
pub fn verify_poc_artifact() -> Result<(), ArtifactError> {
    verify_checksums()?;
    let manifest = parse(MANIFEST)?;
    if manifest["artifact"] != POC_ARTIFACT
        || manifest["protocol_version"] != POC_PROTOCOL_VERSION
        || manifest["schema"] != POC_SCHEMA_SOURCE
        || manifest["schema_digest"] != POC_SCHEMA_DIGEST
        || manifest["provenance"]["source"] != POC_SCHEMA_SOURCE
        || manifest["provenance"]["generator"] != POC_GENERATOR
    {
        return Err(ArtifactError::ManifestMismatch);
    }
    if parse(SCHEMA)?["$id"] != "sts2-poc-v1" {
        return Err(ArtifactError::SchemaMismatch);
    }
    let state = parse(STATE)?;
    let golden = parse(GOLDEN)?;
    let invalid = parse(INVALID)?;
    if state["kind"] != "state_response"
        || golden["kind"] != "action_response"
        || invalid["kind"] != "action_request"
    {
        return Err(ArtifactError::FixtureMismatch);
    }
    Ok(())
}

fn verify_checksums() -> Result<(), ArtifactError> {
    let expected = [
        (
            "schema.json",
            "bec7f808c4b4754c3183eb6f7e83abebdb8e8987545f797cf0f1761114e36cd0",
            SCHEMA.as_bytes(),
        ),
        (
            "manifest.json",
            "9a615af12da0e7edc545c6c7dda647565bd5ae7be9d70d64a3e8cc8a39c87ef0",
            MANIFEST.as_bytes(),
        ),
        (
            "golden/state-response.json",
            "aedc6e99b6697f05ef929d6e209a93c1f1528257051f8ec325e4b3a04e6e35ce",
            STATE.as_bytes(),
        ),
        (
            "golden/action-accepted.json",
            "fa63414eb4fc19860f626e9da68c0831515d3b87e1db0e9bf6942c5ed3864e1c",
            GOLDEN.as_bytes(),
        ),
        (
            "fixtures/invalid-action.json",
            "56b2b377ec4617365f0d4e9f2751b1a0d8a7ba7705134b9a268c98c83a4c4e53",
            INVALID.as_bytes(),
        ),
    ];
    let mut lines = CHECKSUMS.lines();
    for (path, expected_digest, bytes) in expected {
        let line = lines
            .next()
            .ok_or(ArtifactError::ChecksumInventoryMismatch)?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some(expected_digest)
            || fields.next() != Some(path)
            || fields.next().is_some()
        {
            return Err(ArtifactError::ChecksumInventoryMismatch);
        }
        if sha256_hex(bytes) != expected_digest {
            return Err(ArtifactError::ChecksumMismatch);
        }
    }
    if lines.next().is_some() {
        return Err(ArtifactError::ChecksumInventoryMismatch);
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    for byte in digest {
        output.push(char::from(HEX_DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(HEX_DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}

/// A deterministic failure while loading the checked-in release-like artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactError {
    /// A consumed artifact file is not valid JSON.
    InvalidJson,
    /// The manifest does not identify the expected artifact.
    ManifestMismatch,
    /// The schema does not identify the expected contract.
    SchemaMismatch,
    /// The checksum inventory is malformed or unexpected.
    ChecksumInventoryMismatch,
    /// A consumed artifact file does not match its expected checksum.
    ChecksumMismatch,
    /// A consumed fixture has the wrong message kind.
    FixtureMismatch,
}

impl std::fmt::Display for ArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::InvalidJson => "a copied POC artifact file is not valid JSON",
            Self::ManifestMismatch => "the copied POC manifest is not the expected artifact",
            Self::SchemaMismatch => "the copied POC schema is not the expected contract",
            Self::ChecksumInventoryMismatch => "the copied POC checksum inventory is invalid",
            Self::ChecksumMismatch => "a copied POC artifact file has an unexpected checksum",
            Self::FixtureMismatch => "a copied POC fixture has an unexpected message kind",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ArtifactError {}

fn parse(text: &str) -> Result<Value, ArtifactError> {
    serde_json::from_str(text).map_err(|_| ArtifactError::InvalidJson)
}
