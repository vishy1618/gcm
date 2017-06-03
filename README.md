gcm
===

## Usage

Add this to `Cargo.toml`:

```rust
[dependencies]
gcm = "0.2.0"
```

then add this to your crate root:

```rust
extern crate gcm;
```

## Examples:
 
Here is an example to send out a GCM Message with some custom data:
 
```rust
use gcm::Message;
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("message", "Howdy!");

let result = Message::new("<registration id>")
    .data(map)
    .send("<GCM API Key>");
```

To send a message using GCM Notifications, we first build the notification:

```rust
use gcm::{Message, NotificationBuilder};

let notification = NotificationBuilder::new("Hey!")
    .body("Do you want to catch up later?")
    .finalize();
```

And then set it in the message, before sending it:

```rust
let result = Message::new("<registration id>")
    .notification(notification)
    .send("<GCM API Key>");
```

You can now handle the result accordingly:

```rust
match result {
  Ok(response) => println!("message_id: {:?}", response.message_id),
  Err(error) => println!("Error: {:?}", error),
}
```
