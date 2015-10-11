use {Message, Priority};
use GcmError;
use notification::NotificationBuilder;
use std::collections::HashMap;

#[test]
fn should_create_new_message() {
  let msg = Message::new("token");

  assert_eq!(msg.to, "token");
}

#[test]
fn should_set_registration_ids() {
  let msg = Message::new("token");

  assert_eq!(msg.registration_ids, None);

  let msg = Message::new("token")
      .registration_ids(vec!["id1"]);

  assert_eq!(msg.registration_ids, Some(vec!["id1".to_string()]));
}

#[test]
fn should_set_collapse_key() {
  let msg = Message::new("token");

  assert_eq!(msg.collapse_key, None);

  let msg = Message::new("token")
      .collapse_key("key");

  assert_eq!(msg.collapse_key, Some("key"));
}

#[test]
fn should_set_priority() {
  let msg = Message::new("token");

  assert_eq!(msg.priority, None);

  let msg = Message::new("token")
      .priority(Priority::Normal);

  assert_eq!(msg.priority, Some(Priority::Normal));
}

#[test]
fn should_set_content_available() {
  let msg = Message::new("token");

  assert_eq!(msg.content_available, None);

  let msg = Message::new("token")
      .content_available(true);

  assert_eq!(msg.content_available, Some(true));
}

#[test]
fn should_set_delay_while_idle() {
  let msg = Message::new("token");

  assert_eq!(msg.delay_while_idle, None);

  let msg = Message::new("token")
      .delay_while_idle(true);

  assert_eq!(msg.delay_while_idle, Some(true));
}

#[test]
fn should_set_time_to_live() {
  let msg = Message::new("token");

  assert_eq!(msg.time_to_live, None);

  let msg = Message::new("token")
      .time_to_live(10);

  assert_eq!(msg.time_to_live, Some(10));
}

#[test]
fn should_set_restricted_package_name() {
  let msg = Message::new("token");

  assert_eq!(msg.restricted_package_name, None);

  let msg = Message::new("token")
      .restricted_package_name("name");

  assert_eq!(msg.restricted_package_name, Some("name"));
}

#[test]
fn should_set_dry_run() {
  let msg = Message::new("token");

  assert_eq!(msg.dry_run, None);

  let msg = Message::new("token")
      .dry_run(true);

  assert_eq!(msg.dry_run, Some(true));
}

#[test]
fn should_set_data() {
  let msg = Message::new("token");

  assert_eq!(msg.data, None);

  let mut data = HashMap::new();
  data.insert("my", "data");

  let msg = Message::new("token")
      .data(data);

  assert_eq!(msg.data.unwrap().get("my"), Some(&"data".to_string()));
}

#[test]
fn should_set_notifications() {
  let msg = Message::new("token");

  assert_eq!(msg.notification, None);

  let nm = NotificationBuilder::new("title").finalize();

  let msg = Message::new("token")
      .notification(nm);

  assert!(msg.notification != None);
}

#[test]
fn should_parse_error_as_unauthorized() {
  let result = Message::parse_response(401, "Unauthorized");

  assert!(result.is_err());
  assert_eq!(result.err().unwrap(), GcmError::Unauthorized);
}

#[test]
fn should_parse_error_as_invalid_message() {
  let result = Message::parse_response(400, "INVALID_REGISTRATION");

  assert!(result.is_err());
  assert_eq!(result.err().unwrap(), 
    GcmError::InvalidMessage("INVALID_REGISTRATION".to_string()));
}

#[test]
fn should_parse_error_as_server_error() {
  let result = Message::parse_response(500, "Internal Server Error");

  assert!(result.is_err());
  assert_eq!(result.err().unwrap(), GcmError::ServerError);
}

#[test]
fn should_parse_successful_response() {
  let response = r#"
    {
      "message_id": 2000000,
      "results": [
        {
          "message_id": 200000,
          "registration_id": 200000,
          "error": "error"
        }
      ]
    }
  "#;
  let result = Message::parse_response(200, response);

  assert!(result.is_ok());

  let result = result.unwrap();

  assert_eq!(result.message_id.unwrap(), 2000000);

  let message_results = result.results.unwrap();

  assert_eq!(message_results.len(), 1);
}