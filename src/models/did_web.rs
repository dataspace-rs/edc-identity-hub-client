use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Clone, Debug)]
pub struct DidWeb(String);

impl DidWeb {
  pub fn new(url: &str) -> Option<Self> {
    if !url.starts_with("did:web:") {
      return None;
    }

    let extension = if url.matches(":").count() > 2 {
      "/did.json"
    } else {
      "/.well-known/did.json"
    };

    let mut url = url.replace(":", "/").replace("did/web/", "https://");

    url.push_str(extension);

    let url = urlencoding::decode(&url).ok()?.to_string();

    Some(Self(url))
  }

  pub fn url(&self) -> &str {
    &self.0
  }
}

struct DidWebVisitor;
impl<'de> Visitor<'de> for DidWebVisitor {
  type Value = DidWeb;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a DID Web")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: Error,
  {
    DidWeb::new(value).ok_or_else(|| Error::custom(format!("invalid DID Web '{value}'")))
  }
}

impl<'de> Deserialize<'de> for DidWeb {
  fn deserialize<D>(deserializer: D) -> Result<DidWeb, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(DidWebVisitor)
  }
}

#[test]
fn test_did_web() {
  let did_web = DidWeb::new("did:web:w3c-ccg.github.io").unwrap();
  assert_eq!(
    did_web.url(),
    "https://w3c-ccg.github.io/.well-known/did.json"
  );

  let did_web = DidWeb::new("did:web:w3c-ccg.github.io:user:alice").unwrap();
  assert_eq!(
    did_web.url(),
    "https://w3c-ccg.github.io/user/alice/did.json"
  );

  let did_web = DidWeb::new("did:web:example.com%3A3000:user:alice").unwrap();
  assert_eq!(
    did_web.url(),
    "https://example.com:3000/user/alice/did.json"
  );
}
