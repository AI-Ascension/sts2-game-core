// SPDX-License-Identifier: MIT

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Version consumed by this core POC mapping.
pub const POC_PROTOCOL_VERSION: &str = "poc-v1";
/// Schema digest supplied by the protocol release-like artifact.
pub const POC_SCHEMA_DIGEST: &str =
    "242b8f9233e915a55ea8d2e72ca476c1258169a67e62de72ee5aed848a6a0a19";
/// Release-like artifact identity, not a Rust package dependency.
pub const POC_ARTIFACT: &str = "sts2-protocol/poc-v1";

const POC_SCHEMA_FILE: &str = "schema.json";
const POC_SCHEMA_SOURCE: &str = "schemas/poc-v1.schema.json";
const POC_GENERATOR: &str = "hand-authored";
const POC_LICENSE: &str = "MIT";
const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
const MANIFEST: &str = include_str!("../../../protocol-artifact/poc-v1/manifest.json");
const CHECKSUMS: &str = include_str!("../../../protocol-artifact/poc-v1/SHA256SUMS");
const SCHEMA: &str = include_str!("../../../protocol-artifact/poc-v1/schema.json");
const STATE_REQUEST: &str =
    include_str!("../../../protocol-artifact/poc-v1/golden/state-request.json");
const STATE_RESPONSE: &str =
    include_str!("../../../protocol-artifact/poc-v1/golden/state-response.json");
const ACTION_REQUEST: &str =
    include_str!("../../../protocol-artifact/poc-v1/golden/action-request.json");
const ACTION_ACCEPTED: &str =
    include_str!("../../../protocol-artifact/poc-v1/golden/action-accepted.json");
const ACTION_REJECTED: &str =
    include_str!("../../../protocol-artifact/poc-v1/golden/action-rejected.json");
const INVALID: &str =
    include_str!("../../../protocol-artifact/poc-v1/fixtures/invalid-action.json");
const CHECKSUM_ENTRIES: [(&str, &str, &[u8]); 8] = [
    (
        "fixtures/invalid-action.json",
        "29b245f9e0df6c6f158e82e7a770e90e8153b427b3e18e7b00c2340b7a812abf",
        INVALID.as_bytes(),
    ),
    (
        "golden/action-accepted.json",
        "733e4fba7a457bfaf7d1da689369f10974bfde39e4dbae0c1254a6e95ed55a6e",
        ACTION_ACCEPTED.as_bytes(),
    ),
    (
        "golden/action-rejected.json",
        "3c8681361dd87b01969f82aae4ca00f3551e2f07e3215777bba552e2fd4d31ca",
        ACTION_REJECTED.as_bytes(),
    ),
    (
        "golden/action-request.json",
        "0ee20e4b8692e8462288faeacb2f2e78bf986c57d60d89479a31a01cf889286e",
        ACTION_REQUEST.as_bytes(),
    ),
    (
        "golden/state-request.json",
        "46c74fc562031c98f38cc7901f60e06022ec14c6d55b814ae809b571aa58f738",
        STATE_REQUEST.as_bytes(),
    ),
    (
        "golden/state-response.json",
        "816b698fe1d6acd867ef1319d4a51623b9b0d2fa81d82dcfc317c45b6836e2c6",
        STATE_RESPONSE.as_bytes(),
    ),
    (
        "manifest.json",
        "30c8b85a87ff453e9709156ccde65d74722b7c48c0b61a802a28d04277dd3725",
        MANIFEST.as_bytes(),
    ),
    (
        "schema.json",
        "242b8f9233e915a55ea8d2e72ca476c1258169a67e62de72ee5aed848a6a0a19",
        SCHEMA.as_bytes(),
    ),
];

/// Verifies the local copy of the protocol artifact before a POC mapping uses it.
///
/// The checked-in inventory is matched to the eight copied payloads consumed by core, and each
/// payload is hashed before its JSON metadata is inspected. Protocol-owner source and conformance
/// inputs remain outside this consumer-scoped copy.
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
        || manifest["schema"] != POC_SCHEMA_FILE
        || manifest["schema_digest"] != POC_SCHEMA_DIGEST
        || manifest["provenance"]["source"] != POC_SCHEMA_SOURCE
        || manifest["provenance"]["generator"] != POC_GENERATOR
        || manifest["provenance"]["license"] != POC_LICENSE
        || !matches_consumers(&manifest)
    {
        return Err(ArtifactError::ManifestMismatch);
    }
    let schema = parse(SCHEMA)?;
    if sha256_hex(SCHEMA.as_bytes()) != POC_SCHEMA_DIGEST
        || schema["$id"] != "sts2-poc-v1"
        || schema["$defs"]["base"]["properties"]["generation"]["maximum"]
            != json!(9_007_199_254_740_991_u64)
    {
        return Err(ArtifactError::SchemaMismatch);
    }
    for (fixture, expected_kind) in [
        (STATE_REQUEST, "state_request"),
        (STATE_RESPONSE, "state_response"),
        (ACTION_REQUEST, "action_request"),
        (ACTION_ACCEPTED, "action_response"),
        (ACTION_REJECTED, "action_response"),
        (INVALID, "action_request"),
    ] {
        verify_fixture(fixture, expected_kind)?;
    }
    Ok(())
}

fn matches_consumers(manifest: &Value) -> bool {
    let Some(consumers) = manifest["consumers"].as_array() else {
        return false;
    };
    consumers.len() == 5
        && consumers.iter().all(Value::is_string)
        && consumers
            .iter()
            .any(|consumer| consumer.as_str() == Some("sts2-game-core"))
}

fn verify_fixture(text: &str, expected_kind: &str) -> Result<(), ArtifactError> {
    let fixture = parse(text)?;
    let shape_is_valid = match expected_kind {
        "state_request" => {
            fixture["observation"].is_null()
                && fixture["action"].is_null()
                && fixture["status"].is_null()
                && fixture["error_code"].is_null()
        }
        "state_response" => {
            fixture["observation"].is_object()
                && fixture["action"].is_null()
                && fixture["status"].is_null()
                && fixture["error_code"].is_null()
        }
        "action_request" => {
            fixture["observation"].is_null()
                && fixture["action"].is_object()
                && fixture["status"].is_null()
                && fixture["error_code"].is_null()
        }
        "action_response" => {
            fixture["observation"].is_object()
                && fixture["action"].is_object()
                && fixture["status"].is_string()
                && (fixture["error_code"].is_null() || fixture["error_code"].is_string())
        }
        _ => false,
    };
    if fixture["kind"] != expected_kind
        || fixture["protocol_version"] != POC_PROTOCOL_VERSION
        || fixture["schema_digest"] != POC_SCHEMA_DIGEST
        || fixture["provenance"]["artifact"] != POC_ARTIFACT
        || fixture["provenance"]["source"] != POC_SCHEMA_SOURCE
        || fixture["provenance"]["generator"] != POC_GENERATOR
        || !shape_is_valid
    {
        return Err(ArtifactError::FixtureMismatch);
    }
    Ok(())
}

fn verify_checksums() -> Result<(), ArtifactError> {
    let mut lines = CHECKSUMS.lines();
    for (path, expected_digest, bytes) in CHECKSUM_ENTRIES {
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
