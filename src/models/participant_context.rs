use crate::models::IdentityService;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantContext {
  participant_id: String,
  did: String,
  participant_context_id: String,
  key: Key,
  active: bool,
  service_endpoints: Vec<IdentityService>,
  roles: Vec<String>,
  additional_properties: Option<serde_json::Value>,
}

impl ParticipantContext {
  pub fn new(
    participant_id: String,
    participant_context_id: String,
    active: bool,
    service_endpoints: Vec<IdentityService>,
    roles: Vec<String>,
    additional_properties: Option<serde_json::Value>,
  ) -> Self {
    let key = Key {
      key_id: format!("{participant_id}#key-1"),
      private_key_alias: format!("{participant_context_id}#key-1"),
      key_generator_params: KeyGeneratorParams {
        algorithm: "EC".to_string(),
      },
    };

    Self {
      did: participant_id.clone(),
      participant_id,
      participant_context_id,
      key,
      active,
      service_endpoints,
      roles,
      additional_properties,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
  key_id: String,
  private_key_alias: String,
  key_generator_params: KeyGeneratorParams,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyGeneratorParams {
  algorithm: String,
}
