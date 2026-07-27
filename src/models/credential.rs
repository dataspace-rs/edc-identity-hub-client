use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
  pub id: String,
  pub created_at: u64,
  pub timestamp: u64,
  pub issuer_id: String,
  pub holder_id: String,
  pub verifiable_credential: VerifiableCredential,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiableCredential {
  pub raw_vc: String,
  pub format: String,
  pub credential: CredentialClaims,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialClaims {
  pub id: String,
  pub issuance_date: DateTime<Utc>,
  pub expiration_date: DateTime<Utc>,
  pub name: String,
  pub description: Option<String>,
  pub r#type: Vec<String>,
  pub credential_subject: Vec<serde_json::Value>,
}
