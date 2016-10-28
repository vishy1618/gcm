use std::fmt;
use std::error;
use rustc_serialize::json;

#[derive(RustcDecodable, Debug)]
pub struct GcmResponse {
  pub message_id: Option<u64>,
  pub error: Option<String>,
  pub multicast_id: Option<i64>,
  pub success: Option<u64>,
  pub failure: Option<u64>,
  pub canonical_ids: Option<u64>,
  pub results: Option<Vec<MessageResult>>
}

#[derive(RustcDecodable, Debug)]
pub struct MessageResult {
  pub message_id: Option<String>, //can be a string ("fake_message_id") when using dry_run, otherwise its a number
  pub registration_id: Option<u64>,
  pub error: Option<String>
}

#[derive(PartialEq, Debug)]
pub enum GcmError {
  Unauthorized,
  InvalidMessage(String),
  ServerError,
  InvalidJsonBody
}

impl fmt::Display for GcmError {
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

impl From<json::DecoderError> for GcmError {
  fn from(_: json::DecoderError) -> GcmError {
    GcmError::InvalidJsonBody
  }
}