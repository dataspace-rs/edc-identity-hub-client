use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
  pub created_at: u64,
  pub timestamp: u64,
  pub issuer_id: String,
  pub holder_id: String,
  pub verifiable_credential: VerifiableCredential,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiableCredential {
  raw_vc: String,
  format: String,
  credential: serde_json::Value,
}
