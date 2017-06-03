use std::fmt::{self, Display};
use std::error;

use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct GcmResponse {
  pub message_id: Option<u64>,
  pub error: Option<String>,
  pub multicast_id: Option<i64>,
  pub success: Option<u64>,
  pub failure: Option<u64>,
  pub canonical_ids: Option<u64>,
  pub results: Option<Vec<MessageResult>>
}

#[derive(Deserialize, Debug)]
pub struct MessageResult {
  #[serde(deserialize_with = "deserialize_message_id", default)]
  pub message_id: Option<u64>,
  pub registration_id: Option<u64>,
  pub error: Option<String>
}

fn deserialize_message_id<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where D: Deserializer<'de> {
  match u64::deserialize(deserializer) {
    Ok(val) => Ok(Some(val)),
    Err(_) => Ok(None)
  }
}

#[derive(PartialEq, Debug)]
pub enum GcmError {
  Unauthorized,
  InvalidMessage(String),
  ServerError,
  InvalidJsonBody
}

impl Display for GcmError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      GcmError::Unauthorized => write!(f, "UnauthorizedError"),
      GcmError::ServerError => write!(f, "ServerError"),
      GcmError::InvalidMessage(ref message) => write!(f, "InvalidMessage: {}", message),
      GcmError::InvalidJsonBody => write!(f, "InvalidJsonBody")
    }
  }
}

impl error::Error for GcmError {
  fn description(&self) -> &str {
    match *self {
      GcmError::Unauthorized => "UnauthorizedError",
      GcmError::ServerError => "ServerError",
      GcmError::InvalidMessage(_) => "InvalidMessage",
      GcmError::InvalidJsonBody => "InvalidJsonBody"
    }
  }
}
