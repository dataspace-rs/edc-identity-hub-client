use super::{CatalogResponse, IdentityServiceType};
use crate::{DataspaceServiceClient, IssuerServiceClient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityService {
  pub id: String,
  pub r#type: IdentityServiceType,
  pub service_endpoint: String,
}

impl IdentityService {
  pub async fn new(service_endpoint: String) -> Option<Self> {
    let r#type = Self::get_identity_service_type(&service_endpoint).await?;

    Some(Self {
      id: Uuid::new_v4().to_string(),
      r#type,
      service_endpoint,
    })
  }

  pub fn get_issuer_service_client(&self) -> Option<IssuerServiceClient> {
    if self.r#type == IdentityServiceType::IssuerService {
      Some(IssuerServiceClient::new(self.service_endpoint.clone()))
    } else {
      None
    }
  }

  pub fn get_dataspace_service_client(&self) -> Option<DataspaceServiceClient> {
    if self.r#type == IdentityServiceType::DataService {
      DataspaceServiceClient::new(self.service_endpoint.clone())
    } else {
      None
    }
  }

  pub async fn get_identity_service_type(service_endpoint: &str) -> Option<IdentityServiceType> {
    if Self::is_catalog_service(service_endpoint).await {
      Some(IdentityServiceType::CatalogService)
    } else if Self::is_data_service(service_endpoint).await {
      Some(IdentityServiceType::DataService)
    } else if Self::is_issuer_service(service_endpoint)
      .await
      .unwrap_or(false)
    {
      Some(IdentityServiceType::IssuerService)
    } else {
      None
    }
  }

  pub async fn is_catalog_service(service_endpoint: &str) -> bool {
    let client = reqwest::Client::new();
    if let Ok(response) = client
      .post(format!("{service_endpoint}/catalog/request"))
      .json(&serde_json::json!(
        {
          "@context": [
              "https://w3id.org/dspace/2025/1/context.jsonld"
            ],
            "@type": "CatalogRequestMessage",
            "filter": []
          }
      ))
      .send()
      .await
    {
      response.json::<CatalogResponse>().await.is_ok()
    } else {
      false
    }
  }

  pub async fn is_data_service(service_endpoint: &str) -> bool {
    if let Some(dataspace_service_client) =
      DataspaceServiceClient::new(service_endpoint.to_string())
    {
      dataspace_service_client
        .get_protocol_versions()
        .await
        .is_ok()
    } else {
      false
    }
  }

  pub async fn is_issuer_service(service_endpoint: &str) -> reqwest::Result<bool> {
    let _ = IssuerServiceClient::new(service_endpoint.to_string())
      .get_metadata()
      .await?;

    Ok(true)
  }
}
