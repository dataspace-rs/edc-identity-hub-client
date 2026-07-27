#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestCredentialInformation {
  pub issuer_did: String,
  pub holder_pid: String,
  pub issuer_pid: String,
  pub status: RequestCredentialState,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestCredentialState {
  Initial,
  Requesting,
  Requested,
  Issuing,
  Issued,
  Revoked,
  Suspended,
  Expired,
  #[serde(rename = "NOT_YET_VALID")]
  NotYetValid,
  Error,
}
