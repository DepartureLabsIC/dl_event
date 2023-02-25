# Departure Labs Simple Events 

A simple and fun library for publishing canister events to subscribers! 🎉


> Please note that our implementation is not optimized for 
> pushing large volumes of events or retrying on failures. 
> However, it is still a great option for canisters with a few subscribers 
> who need to share non-critical information.
> Got that? Okay, let's get started!


## Publishing Events

To publish an event, first derive `CandidType`, `Serialize`, `Deserialize` traits for your event struct, and then implement `SimpleEvent` trait on it.

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

Then, add a subscriber using the add_subscriber function:

```rust
dl_events::add_subscriber(Principal::anonymous());
```

Finally, publish an event using the publish_event function:

```rust
dl_events::publish_event(&FriendMessagedEvent {
        message: "Hello World".to_string(),
        at: 123
    });
```

## Consuming Events

To consume events, add the following update method to your canister:

```rust

#[update]
fn dl_events_simple_notify_v0(event : dl_events::Event) -> () {
    
}
```

Next, implement logic to determine the type of incoming event:

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

