#[cfg(test)]
mod tests;
pub mod response;

pub use message::response::*;
use notification::Notification;
use std::collections::HashMap;
use std::str;
use std::io::Read;

use hyper::Client;
use hyper::header;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::status::{StatusCode,StatusClass};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::{from_str, to_string};
use serde::{Serializer};

#[derive(PartialEq, Debug, Serialize)]
pub enum Priority {
  Normal,
  High
}

/// Represents a GCM message. Construct the GCM message 
/// using various utility methods and finally send it.
/// # Examples:
/// ```rust
/// use gcm::Message;
/// 
/// let message = Message::new("<registration id>").dry_run(true);
/// ```
#[derive(Serialize)]
pub struct Message<'a> {
  to: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  registration_ids: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  collapse_key: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none", serialize_with = "priority_lowercase")]
  priority: Option<Priority>,
  #[serde(skip_serializing_if = "Option::is_none")]
  content_available: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  delay_while_idle: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  time_to_live: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  restricted_package_name: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  dry_run: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  data: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  notification: Option<Notification<'a>>,
}

fn priority_lowercase<S>(priority_field: &Option<Priority>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
  // unwrapping cause we skip serializing if none
  let normal_priority = Priority::Normal;
  let priority = priority_field.as_ref().unwrap_or(&normal_priority);
  match *priority {
    Priority::Normal => serializer.serialize_str("normal"),
    Priority::High => serializer.serialize_str("high")
  }
}

impl <'a> Message<'a> {
  /// Get a new instance of Message. You need to supply either
  /// a registration id, or a topic (/topic/...).
  pub fn new(to: &'a str) -> Message {
    Message {
      to: to,
      registration_ids: None,
      collapse_key: None,
      priority: None,
      content_available: None,
      delay_while_idle: None,
      time_to_live: None,
      restricted_package_name: None,
      dry_run: None,
      data: None,
      notification: None,
    }
  }

  /// Set various registration ids to which the message ought to be sent.
  pub fn registration_ids(mut self, ids: Vec<&'a str>) -> Message<'a> {
    self.registration_ids = Some(ids.iter().map(|s| s.to_string()).collect());
    self
  }

  /// Set this parameter to identify groups of messages that can be collapsed.
  pub fn collapse_key(mut self, collapse_key: &'a str) -> Message<'a> {
    self.collapse_key = Some(collapse_key);
    self
  }

  /// Set the priority of the message. You can set Normal or High priorities.
  /// # Examples:
  /// ```rust
  /// use gcm::{Message, Priority};
  /// 
  /// let message = Message::new("<registration id>")
  ///     .priority(Priority::High);
  /// ```
  pub fn priority(mut self, priority: Priority) -> Message<'a> {
    self.priority = Some(priority);
    self
  }

  /// To set the `content-available` field on iOS
  pub fn content_available(mut self, content_available: bool) -> Message<'a> {
    self.content_available = Some(content_available);
    self
  }

  /// When set to `true`, sends the message only when the device is active.
  pub fn delay_while_idle(mut self, delay_while_idle: bool) -> Message<'a> {
    self.delay_while_idle = Some(delay_while_idle);
    self
  }

  /// How long (in seconds) to keep the message on GCM servers in case the device 
  /// is offline. The maximum and default is 4 weeks.
  pub fn time_to_live(mut self, time_to_live: i32) -> Message<'a> {
    self.time_to_live = Some(time_to_live);
    self
  }

  /// Package name of the application where the registration tokens must match.
  pub fn restricted_package_name(mut self, restricted_package_name: &'a str) -> Message<'a> {
    self.restricted_package_name = Some(restricted_package_name);
    self
  }

  /// When set to `true`, allows you to test GCM without actually sending the message.
  pub fn dry_run(mut self, dry_run: bool) -> Message<'a> {
    self.dry_run = Some(dry_run);
    self
  }

  /// Use this to add custom key-value pairs to the message. This data
  /// must be handled appropriately on the client end.
  /// # Examples:
  /// ```rust
  /// use gcm::Message;
  /// use std::collections::HashMap;
  ///
  /// let mut map = HashMap::new();
  /// map.insert("message", "Howdy!");
  /// 
  /// let message = Message::new("<registration id>").data(map);
  /// ```
  pub fn data(mut self, data: HashMap<&'a str, &'a str>) -> Message<'a> {
    let mut datamap: HashMap<String, String> = HashMap::new();
    for (key, val) in data.iter() {
      datamap.insert(key.to_string(), val.to_string());
    }

    self.data = Some(datamap);
    self
  }

  /// Use this to set a `Notification` for the message.
  /// # Examples:
  /// ```rust
  /// use gcm::{Message, NotificationBuilder};
  ///
  /// let notification = NotificationBuilder::new("Hey!")
  ///     .body("Do you want to catch up later?")
  ///     .finalize();
  /// 
  /// let message = Message::new("<registration id>")
  ///     .notification(notification);
  /// ```
  pub fn notification(mut self, notification: Notification<'a>) -> Message<'a> {
    self.notification = Some(notification);
    self
  }

  /// Send the message using your GCM API Key.
  /// # Examples:
  /// ```no_run
  /// use gcm::Message;
  /// use std::collections::HashMap;
  ///
  /// let mut map = HashMap::new();
  /// map.insert("message", "Howdy!");
  /// 
  /// let result = Message::new("<registration id>")
  ///     .data(map)
  ///     .send("<GCM API Key>");
  /// ```
  pub fn send(self, api_key: &'a str) -> Result<GcmResponse, GcmError> {
  	let ssl = NativeTlsClient::new().unwrap();
  	let connector = HttpsConnector::new(ssl);
  	let client = Client::with_connector(connector);
    let json_body;

    match to_string(&self) {
      Ok(body) => {json_body = body;},
      Err(_) => {return Err(GcmError::InvalidJsonBody);}
    };

  	let result = client.post("https://gcm-http.googleapis.com/gcm/send")
  					.body(json_body.as_bytes())
  					.header(header::Authorization("key=".to_string() + api_key))
  					.header(
              header::ContentType(
                Mime(
                  TopLevel::Application,
                  SubLevel::Json,
                  vec![(Attr::Charset, Value::Utf8)]
                )
              )
            )
  					.send();

    match result {
      Ok(mut res) => {
        let mut body = String::new();
        match res.read_to_string(&mut body) {
          Ok(_) => Message::parse_response(res.status, &body),
          Err(_) => Message::parse_response(StatusCode::InternalServerError, "Server Error")
        }
      },
      Err(_) => {
        Message::parse_response(StatusCode::InternalServerError, "Server Error")
      }
    }
  }

  fn parse_response(status: StatusCode, body: &str) -> Result<GcmResponse, GcmError> {
  	//200 Ok: Request was successful!
  	if status == StatusCode::Ok {
      return from_str(body).or_else(|_| Err(GcmError::InvalidJsonBody));
  	}
  	//check for server error (5xx)
  	if status.class() == StatusClass::ServerError {
  		return Err(GcmError::ServerError);
  	}
  	//match remaining status codes
  	match status {
  		StatusCode::Unauthorized => Err(GcmError::Unauthorized),
  		StatusCode::BadRequest => Err(GcmError::InvalidMessage(body.to_string())),
  		_ => Err(GcmError::InvalidMessage("Unknown Error".to_string()))
  	}
  }
}
