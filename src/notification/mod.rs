#[cfg(test)]
mod tests;

/// This struct represents a GCM notification. Use the 
/// corresponding `NotificationBuilder` to get an instance. You can then use 
/// this notification instance when sending a GCM message.
#[derive(Debug, PartialEq, Serialize)]
pub struct Notification<'a> {
  title: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  body: Option<&'a str>,
  icon: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  sound: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  badge: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  tag: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  color: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  click_action: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  body_loc_key: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  body_loc_args: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  title_loc_key: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  title_loc_args: Option<Vec<String>>,
}

/// A builder to get a `Notification` instance.
///
/// # Examples
///
/// ```rust
/// use gcm::NotificationBuilder;
///
/// let notification = NotificationBuilder::new("India vs. Australia")
///     .body("3 runs to win in 1 ball")
///     .finalize();
/// ```
pub struct NotificationBuilder<'a> {
  title: &'a str,
  body: Option<&'a str>,
  icon: &'a str,
  sound: Option<&'a str>,
  badge: Option<&'a str>,
  tag: Option<&'a str>,
  color: Option<&'a str>,
  click_action: Option<&'a str>,
  body_loc_key: Option<&'a str>,
  body_loc_args: Option<Vec<String>>,
  title_loc_key: Option<&'a str>,
  title_loc_args: Option<Vec<String>>,
}

impl <'a> NotificationBuilder<'a> {
  /// Get a new `NotificationBuilder` instance, with a title.
  pub fn new(title: &'a str) -> NotificationBuilder<'a> {
    NotificationBuilder {
      title: title,
      body: None,
      icon: "myicon",
      sound: None,
      badge: None,
      tag: None,
      color: None,
      click_action: None,
      body_loc_key: None,
      body_loc_args: None,
      title_loc_key: None,
      title_loc_args: None,
    }
  }

  /// Set the body of the notification
  pub fn body(&mut self, body: &'a str) -> &mut NotificationBuilder<'a> {
    self.body = Some(body);
    self
  }

  /// Set the notification icon. Defaults to `myicon`
  pub fn icon(&mut self, icon: &'a str) -> &mut NotificationBuilder<'a> {
    self.icon = icon;
    self
  }

  /// Set the sound to be played
  pub fn sound(&mut self, sound: &'a str) -> &mut NotificationBuilder<'a> {
    self.sound = Some(sound);
    self
  }

  /// Set the badge for iOS notifications
  pub fn badge(&mut self, badge: &'a str) -> &mut NotificationBuilder<'a> {
    self.badge = Some(badge);
    self
  }

  /// Tagging a notification allows you to replace existing notifications
  /// with the same tag with this new notification
  pub fn tag(&mut self, tag: &'a str) -> &mut NotificationBuilder<'a> {
    self.tag = Some(tag);
    self
  }

  /// The color of the icon, in #rrggbb format
  pub fn color(&mut self, color: &'a str) -> &mut NotificationBuilder<'a> {
    self.color = Some(color);
    self
  }

  /// What happens when the user clicks on the notification. Refer to 
  /// https://developers.google.com/cloud-messaging/http-server-ref#table2 for
  /// details.
  pub fn click_action(&mut self, click_action: &'a str) -> &mut NotificationBuilder<'a> {
    self.click_action = Some(click_action);
    self
  }

  /// Set the body key string for localization
  pub fn body_loc_key(&mut self, body_loc_key: &'a str) -> &mut NotificationBuilder<'a> {
    self.body_loc_key = Some(body_loc_key);
    self
  }

  /// String value to replace format specifiers in the body string.
  pub fn body_loc_args(&mut self, body_loc_args: Vec<&'a str>) -> &mut NotificationBuilder<'a> {
    self.body_loc_args = Some(body_loc_args.iter().map(|s| s.to_string()).collect());
    self
  }

  /// Set the title key string for localization
  pub fn title_loc_key(&mut self, title_loc_key: &'a str) -> &mut NotificationBuilder<'a> {
    self.title_loc_key = Some(title_loc_key);
    self
  }

  /// String value to replace format specifiers in the title string.
  pub fn title_loc_args(&mut self, title_loc_args: Vec<&'a str>) -> &mut NotificationBuilder<'a> {
    self.title_loc_args = Some(title_loc_args.iter().map(|s| s.to_string()).collect());
    self
  }

  /// Complete the build and get a `Notification` instance
  pub fn finalize(&mut self) -> Notification<'a> {
    Notification {
      title: self.title,
      body: self.body,
      icon: self.icon,
      sound: self.sound,
      badge: self.badge,
      tag: self.tag,
      color: self.color,
      click_action: self.click_action,
      body_loc_key: self.body_loc_key,
      body_loc_args: self.body_loc_args.clone(),
      title_loc_key: self.title_loc_key,
      title_loc_args: self.title_loc_args.clone(),
    }
  }
}