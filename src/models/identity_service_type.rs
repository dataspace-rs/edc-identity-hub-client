use serde::{Deserialize, Serializer};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum IdentityServiceType {
  CatalogService,
  DataService,
  IssuerService,
  Custom(String),
}

impl serde::Serialize for IdentityServiceType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let value = match self {
      IdentityServiceType::CatalogService => "CatalogService".to_string(),
      IdentityServiceType::DataService => "DataService".to_string(),
      IdentityServiceType::IssuerService => "IssuerService".to_string(),
      IdentityServiceType::Custom(content) => content.to_string(),
    };

    serializer.serialize_str(&value)
  }
}
