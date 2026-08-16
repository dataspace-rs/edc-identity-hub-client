use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum IdentityServiceType {
  CatalogService,
  DataService,
  IssuerService,
  Custom(String),
}
