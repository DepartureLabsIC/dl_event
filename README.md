# Departure Labs Simple Events 

This is a simple library for publishing canister events subscribers. 


## Publishing Events


First implement the SimpleEvent trait for your event

```rust
#[derive(CandidType, Serialize, Deserialize)]
pub struct FriendMessagedEvent {
    message : String,
    at : u64,
}

impl SimpleEvent for FriendMessagedEvent {
    // Events are useless if receivers can't interpret them
    fn event_schema(&self) -> String {
        "some_github_link".into()
    }
    // Best practice use a versioned hierarchical naming scheme here
    fn event_type(&self) -> String {
        "dl.v0.friend.message".into()
    }
    
    fn created_at(&self) -> u64 {
        self.at
    }
}

```

Add a subscriber using the `add_subscriber` function

```rust
dl_events::add_subscriber(Principal::anonymous());
```

finally, publish an event

```rust
dl_events::publish_event(&FriendMessagedEvent {
        message: "Hello World".to_string(),
        at: 123
    });
```

## Consuming Events

add the following update of the following shape to your canister

```rust

#[update]
fn dl_events_simple_notify_v0(event : dl_events::Event) -> () {
    
}
```

Next, implement logic to determine type of incoming event


```rust
#[update]
fn dl_events_simple_notify_v0(event : dl_events::Event) -> () {
    match event.event_type() {
        "dl.v0.friend.message" => {
            if let Ok(friend_message) = event.into::<FriendMessagedEvent>() {
                // Yay!
            }
        },
        &_ => {}
    }
}
```