use crate::models::DidWeb;
use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
  pub participant_context_id: String,
  pub state: u16,
  pub did: DidWeb,
  #[serde(with = "ts_milliseconds")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "ts_milliseconds")]
  pub last_modified: DateTime<Utc>,
}
