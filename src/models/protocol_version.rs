use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolVersion {
  pub version: Version,
  pub path: String,
  pub binding: Binding,
  pub auth: Option<Auth>,
  pub identifier_type: Option<String>,
  pub service_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
  protocol: String,
  version: String,
  profile: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Version {
  #[serde(rename = "0.8")]
  _0_8,
  #[serde(rename = "2024-1")]
  _2024_1,
  #[serde(rename = "2025-1")]
  _2025_1,
  #[serde(untagged)]
  Unknown(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Binding {
  Https,
}

#[test]
fn test_protocol_version() {
  let data = serde_json::json!({
    "version": "0.8",
    "path": "/",
    "binding": "HTTPS"
  });

  let protocol_version = serde_json::from_value::<ProtocolVersion>(data);

  assert!(protocol_version.is_ok());

  let data = serde_json::json!({
    "version": "2024-1",
    "path": "/2024-1",
    "binding": "HTTPS"
  });

  let protocol_version = serde_json::from_value::<ProtocolVersion>(data);

  assert!(protocol_version.is_ok());

  let data = serde_json::json!({
    "version": "2025-1",
    "path": "/2025-1",
    "binding": "HTTPS"
  });

  let protocol_version = serde_json::from_value::<ProtocolVersion>(data);

  assert!(protocol_version.is_ok());

  let data = serde_json::json!({
    "version": "unknown-version",
    "path": "/custom/path",
    "binding": "HTTPS"
  });

  let protocol_version = serde_json::from_value::<ProtocolVersion>(data);

  assert!(protocol_version.is_ok());
}
