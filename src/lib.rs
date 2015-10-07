//! gcm
//! ===
//!
//! # Usage:
//!
//! Add this to `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! gcm = "0.1.0"
//! ```
//!
//! then add this to your crate root:
//!
//! ```ignore
//! extern crate gcm;
//! ```
//!
//! # Examples:
//! 
//! Here is an example to send out a GCM Message with some custom data:
//! 
//! ```no_run
//! use gcm::message::Message;
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! map.insert("message", "Howdy!");
//! 
//! let result = Message::new("<registration id>")
//!     .data(map)
//!     .send("<GCM API Key>");
//! ```
//!
//! To send a message using GCM Notifications, we first build the notification:
//! 
//! ```rust
//! use gcm::message::Message;
//! use gcm::notification::NotificationBuilder;
//!
//! let notification = NotificationBuilder::new("Hey!")
//!     .body("Do you want to catch up later?")
//!     .finalize();
//! ```
//! And then set it in the message, before sending it:
//! 
//! ```no_run
//! # use gcm::message::Message;
//! # use gcm::notification::NotificationBuilder;
//! # let notification = NotificationBuilder::new("Hey!")
//! #     .body("Do you want to catch up later?")
//! #     .finalize();
//! let message = Message::new("<registration id>")
//!     .notification(notification)
//!     .send("<GCM API Key>");
//! ```

pub mod message;
pub mod notification;

extern crate rustc_serialize;
extern crate curl;