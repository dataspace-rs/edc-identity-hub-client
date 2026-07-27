pub type Result<T> = std::result::Result<T, IdentityHubClientError>;

#[derive(Debug)]
pub enum IdentityHubClientError {
  Reqwest(reqwest::Error),
  Response(reqwest::Response),
}

impl From<reqwest::Error> for IdentityHubClientError {
  fn from(err: reqwest::Error) -> Self {
    Self::Reqwest(err)
  }
}
