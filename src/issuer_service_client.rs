use crate::models::IssuerServiceMetadata;

pub struct IssuerServiceClient {
  service_endpoint: String,
}

impl IssuerServiceClient {
  pub fn new(service_endpoint: String) -> Self {
    Self { service_endpoint }
  }

  pub async fn get_metadata(&self) -> reqwest::Result<IssuerServiceMetadata> {
    reqwest::get(format!("{}/metadata", self.service_endpoint))
      .await?
      .json()
      .await
  }
}
