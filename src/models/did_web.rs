pub struct DidWeb {
  url: String,
}

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

    Some(Self { url })
  }

  pub fn url(&self) -> &str {
    &self.url
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
