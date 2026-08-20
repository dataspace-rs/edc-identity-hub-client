use crate::models::ProtocolVersion;
use serde::Deserialize;

pub struct DataspaceServiceClient {
  service_endpoint: String,
}

impl DataspaceServiceClient {
  pub fn new(service_endpoint: String) -> Option<Self> {
    if service_endpoint.ends_with("/.well-known/dspace-version") {
      Some(Self { service_endpoint })
    } else {
      None
    }
  }

  pub async fn get_protocol_versions(&self) -> reqwest::Result<Vec<ProtocolVersion>> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ResponseBody {
      protocol_versions: Vec<ProtocolVersion>,
    }

    let response_body = reqwest::get(&self.service_endpoint)
      .await?
      .json::<ResponseBody>()
      .await?;

    Ok(response_body.protocol_versions)
  }

  pub async fn get_first_service_endpoint(&self) -> Option<String> {
    let protocol_versions = self.get_protocol_versions().await.ok()?;

    protocol_versions.first().map(|protocol_version| {
      self
        .service_endpoint
        .replace("/.well-known/dspace-version", &protocol_version.path)
    })
  }
}
