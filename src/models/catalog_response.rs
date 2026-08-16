use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatalogResponse {
  #[serde(rename = "@context")]
  context: Vec<String>,
  #[serde(rename = "@type")]
  r#type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CatalogResponseType {
  Catalog,
  CatalogError,
}
