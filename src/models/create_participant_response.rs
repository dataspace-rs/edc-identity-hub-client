use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParticipantResponse {
  pub api_key: String,
  pub client_id: String,
  pub client_secret: String,
}
