#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestCredentialBody {
  issuer_did: String,
  holder_pid: String,
  credentials: Vec<CredentialQuery>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialQuery {
  format: CredentialFormat,
  r#type: String,
  id: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CredentialFormat {
  #[serde(rename = "VC1_0_JWT")]
  Vc10Jwt,
}
