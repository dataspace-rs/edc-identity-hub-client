#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestCredentialBody {
  pub issuer_did: String,
  pub holder_pid: String,
  pub credentials: Vec<CredentialQuery>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialQuery {
  pub format: CredentialFormat,
  pub r#type: String,
  pub id: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CredentialFormat {
  #[serde(rename = "VC1_0_JWT")]
  Vc10Jwt,
}
