use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuerServiceMetadata {
  pub r#type: String,
  pub issuer: String,
  pub credentials_supported: Vec<CredentialsSupported>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CredentialsSupported {
  pub id: String,
  pub r#type: String,
  pub offer_reason: String,
  pub profile: String,
  pub binding_methods: Vec<String>,
  pub credential_type: String,
  pub issuance_policy: InsurancePolicy,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePolicy {
  pub id: String,
}
