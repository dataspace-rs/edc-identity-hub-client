use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

static CATALOG_SERVICE_ID: &str = "CatalogService";
static DATA_SERVICE_ID: &str = "DataService";
static ISSUER_SERVICE_ID: &str = "IssuerService";

#[derive(Clone, Debug, PartialEq)]
pub enum IdentityServiceType {
  CatalogService,
  DataService,
  IssuerService,
  Custom(String),
}

impl FromStr for IdentityServiceType {
  type Err = ();

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    let identity_service_type = if value == CATALOG_SERVICE_ID {
      IdentityServiceType::CatalogService
    } else if value == DATA_SERVICE_ID {
      IdentityServiceType::DataService
    } else if value == ISSUER_SERVICE_ID {
      IdentityServiceType::IssuerService
    } else {
      IdentityServiceType::Custom(value.to_string())
    };

    Ok(identity_service_type)
  }
}

impl Display for IdentityServiceType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let identity_service_type = match self {
      IdentityServiceType::CatalogService => CATALOG_SERVICE_ID.to_string(),
      IdentityServiceType::DataService => DATA_SERVICE_ID.to_string(),
      IdentityServiceType::IssuerService => ISSUER_SERVICE_ID.to_string(),
      IdentityServiceType::Custom(content) => content.to_string(),
    };

    write!(f, "{}", identity_service_type)
  }
}

impl serde::Serialize for IdentityServiceType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

impl<'de> Deserialize<'de> for IdentityServiceType {
  fn deserialize<D>(deserializer: D) -> Result<IdentityServiceType, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(IdentityServiceTypeVisitor)
  }
}

struct IdentityServiceTypeVisitor;

impl<'de> Visitor<'de> for IdentityServiceTypeVisitor {
  type Value = IdentityServiceType;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("an string containing a valid identity service type")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: Error,
  {
    Self::Value::from_str(value).map_err(|_| E::custom("invalid identity service type"))
  }
}
