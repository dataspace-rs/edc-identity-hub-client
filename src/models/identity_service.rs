use crate::IssuerServiceClient;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityService {
  pub id: String,
  pub r#type: String,
  pub service_endpoint: String,
}

impl IdentityService {
  pub fn get_issuer_service_client(&self) -> Option<IssuerServiceClient> {
    if self.r#type == "IssuerService" {
      Some(IssuerServiceClient::new(self.service_endpoint.clone()))
    } else {
      None
    }
  }
}
