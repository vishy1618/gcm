#[cfg(test)]
mod tests;
pub mod response;

use notification::Notification;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::str;
use curl::http;
use rustc_serialize::json::{self, Json, ToJson};

#[derive(PartialEq, Debug, Clone)]
pub enum Priority {
  Normal, High
}

pub struct Message<'a> {
  to: &'a str,
  registration_ids: Option<Vec<String>>,
  collapse_key: Option<&'a str>,
  priority: Option<Priority>,
  content_available: Option<bool>,
  delay_while_idle: Option<bool>,
  time_to_live: Option<i32>,
  restricted_package_name: Option<&'a str>,
  dry_run: Option<bool>,
  data: Option<HashMap<String, String>>,
  notification: Option<Notification<'a>>,
}

impl <'a> ToJson for Message<'a> {
  fn to_json(&self) -> Json {
    let mut root = BTreeMap::new();

    root.insert("to".to_string(), self.to.to_json());

    if self.registration_ids.is_some() {
      root.insert("registration_ids".to_string(), 
          self.registration_ids.clone().unwrap().to_json());
    }

    if self.collapse_key.is_some() {
      root.insert("collapse_key".to_string(), self.collapse_key.clone().unwrap().to_json());
    }

    if self.priority.is_some() {
      root.insert("priority".to_string(), match self.priority.clone().unwrap() {
        Priority::Normal => Json::String("normal".to_string()),
        Priority::High => Json::String("high".to_string()),
      });
    }

    if self.content_available.is_some() {
      root.insert("content_available".to_string(), self.content_available.unwrap().to_json());
    }

    if self.delay_while_idle.is_some() {
      root.insert("delay_while_idle".to_string(), self.delay_while_idle.unwrap().to_json());
    }

    if self.time_to_live.is_some() {
      root.insert("time_to_live".to_string(), self.time_to_live.unwrap().to_json());
    }

    if self.restricted_package_name.is_some() {
      root.insert("restricted_package_name".to_string(), self.restricted_package_name.clone().unwrap().to_json());
    }

    if self.dry_run.is_some() {
      root.insert("dry_run".to_string(), self.dry_run.unwrap().to_json());
    }

    if self.data.is_some() {
      root.insert("data".to_string(), self.data.clone().unwrap().to_json());
    }

    if self.notification.is_some() {
      root.insert("notification".to_string(), self.notification.clone().unwrap().to_json());
    }

    Json::Object(root)
  }
}

impl <'a> Message<'a> {
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

  pub fn registration_ids(mut self, ids: Vec<&'a str>) -> Message<'a> {
    self.registration_ids = Some(ids.iter().map(|s| s.to_string()).collect());
    self
  }

  pub fn collapse_key(mut self, collapse_key: &'a str) -> Message<'a> {
    self.collapse_key = Some(collapse_key);
    self
  }

  pub fn priority(mut self, priority: Priority) -> Message<'a> {
    self.priority = Some(priority);
    self
  }

  pub fn content_available(mut self, content_available: bool) -> Message<'a> {
    self.content_available = Some(content_available);
    self
  }

  pub fn delay_while_idle(mut self, delay_while_idle: bool) -> Message<'a> {
    self.delay_while_idle = Some(delay_while_idle);
    self
  }

  pub fn time_to_live(mut self, time_to_live: i32) -> Message<'a> {
    self.time_to_live = Some(time_to_live);
    self
  }

  pub fn restricted_package_name(mut self, restricted_package_name: &'a str) -> Message<'a> {
    self.restricted_package_name = Some(restricted_package_name);
    self
  }

  pub fn dry_run(mut self, dry_run: bool) -> Message<'a> {
    self.dry_run = Some(dry_run);
    self
  }

  pub fn data(mut self, data: HashMap<&'a str, &'a str>) -> Message<'a> {
    let mut datamap: HashMap<String, String> = HashMap::new();
    for (key, val) in data.iter() {
      datamap.insert(key.to_string(), val.to_string());
    }

    self.data = Some(datamap);
    self
  }

  pub fn notification(mut self, notification: Notification<'a>) -> Message<'a> {
    self.notification = Some(notification);
    self
  }

  pub fn send(self, api_key: &'a str) -> Result<response::GcmResponse, response::GcmError> {
    let payload = self.to_json().to_string();
    let auth_header = "key=".to_string() + api_key;
    let res;
    let body;
    let code;

    res = http::handle()
        .post("https://gcm-http.googleapis.com/gcm/send", &payload)
        .header("Authorization", &auth_header)
        .header("Content-Type", "application/json")
        .exec()
        .unwrap();

    body = str::from_utf8(res.get_body()).unwrap();
    code = res.get_code();

    Message::parse_response(code, body)
  }

  fn parse_response(status: u32, body: &str) -> Result<response::GcmResponse, response::GcmError> {
    match status {
      200 => {
        Ok(json::decode(body).unwrap())
      },
      401 => Err(response::GcmError::Unauthorized),
      400 => Err(response::GcmError::InvalidMessage(body.to_string())),
      500...599 => Err(response::GcmError::ServerError),
      _ => Err(response::GcmError::InvalidMessage("Unknown Error".to_string())),
    }
  }
}