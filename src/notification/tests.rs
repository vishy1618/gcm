use serde_json;
use {NotificationBuilder};

#[test]
fn should_create_new_notification_message() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.title, "title");
}

#[test]
fn should_set_notification_body() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.body, None);

  let nm = NotificationBuilder::new("title")
      .body("body")
      .finalize();

  let json_result = serde_json::to_string(&nm);

  assert_eq!(nm.body, Some("body"));
  assert!(json_result.is_ok());
  assert_eq!(json_result.unwrap(), r#"{"title":"title","body":"body","icon":"myicon"}"#);
}

#[test]
fn should_set_default_icon() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.icon, "myicon");
}

#[test]
fn should_set_notification_icon() {
  let nm = NotificationBuilder::new("title")
      .icon("newicon")
      .finalize();

  assert_eq!(nm.icon, "newicon");
}

#[test]
fn should_set_notification_sound() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.sound, None);

  let nm = NotificationBuilder::new("title")
      .sound("sound.wav")
      .finalize();

  assert_eq!(nm.sound, Some("sound.wav"));
}

#[test]
fn should_set_notification_badge() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.badge, None);

  let nm = NotificationBuilder::new("title")
      .badge("1")
      .finalize();

  assert_eq!(nm.badge, Some("1"));
}

#[test]
fn should_set_notification_tag() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.tag, None);

  let nm = NotificationBuilder::new("title")
      .tag("tag")
      .finalize();

  assert_eq!(nm.tag, Some("tag"));
}

#[test]
fn should_set_notification_color() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.color, None);

  let nm = NotificationBuilder::new("title")
      .color("color")
      .finalize();

  assert_eq!(nm.color, Some("color"));
}

#[test]
fn should_set_notification_click_action() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.click_action, None);

  let nm = NotificationBuilder::new("title")
      .click_action("action")
      .finalize();

  assert_eq!(nm.click_action, Some("action"));
}

#[test]
fn should_set_notification_body_loc_key() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.body_loc_key, None);

  let nm = NotificationBuilder::new("title")
      .body_loc_key("key")
      .finalize();

  assert_eq!(nm.body_loc_key, Some("key"));
}

#[test]
fn should_set_notification_body_loc_args() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.body_loc_args, None);

  let nm = NotificationBuilder::new("title")
      .body_loc_args(vec!["args"])
      .finalize();

  let json_result = serde_json::to_string(&nm);

  assert_eq!(nm.body_loc_args, Some(vec!["args".to_string()]));
  assert_eq!(json_result.unwrap(), r#"{"title":"title","icon":"myicon","body_loc_args":["args"]}"#);
}

#[test]
fn should_set_notification_title_loc_key() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.title_loc_key, None);

  let nm = NotificationBuilder::new("title")
      .title_loc_key("key")
      .finalize();

  assert_eq!(nm.title_loc_key, Some("key"));
}

#[test]
fn should_set_notification_title_loc_args() {
  let nm = NotificationBuilder::new("title").finalize();

  assert_eq!(nm.title_loc_args, None);

  let nm = NotificationBuilder::new("title")
      .title_loc_args(vec!["args"])
      .finalize();

  let json_result = serde_json::to_string(&nm);

  assert_eq!(nm.title_loc_args, Some(vec!["args".to_string()]));
  assert_eq!(json_result.unwrap(), r#"{"title":"title","icon":"myicon","title_loc_args":["args"]}"#);
}