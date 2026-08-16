use super::{IdentityService, IdentityServiceType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Identity {
  pub id: String,
  #[serde(rename = "service")]
  pub services: Vec<IdentityService>,
}

impl Identity {
  pub fn get_identity_services(&self, r#type: IdentityServiceType) -> Vec<&IdentityService> {
    self
      .services
      .iter()
      .filter(|service| service.r#type == r#type)
      .collect()
  }
}
