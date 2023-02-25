use candid::Principal;
use dl_events::SimpleEvent;
use ic_cdk::export::candid::CandidType;
use serde::{Serialize, Deserialize};

#[derive(CandidType, Serialize, Deserialize)]
pub struct FriendMessagedEvent {
    message : String,
    at : u64,
}

impl SimpleEvent for FriendMessagedEvent {
    fn event_schema(&self) -> String {
        "some_github_link".into()
    }

    fn event_type(&self) -> String {
        "dl.v0.friend.message".into()
    }

    fn created_at(&self) -> u64 {
        self.at
    }
}

fn add_subscriber() {
    dl_events::add_subscriber(Principal::anonymous());
}

fn main() {
    dl_events::publish_event(&FriendMessagedEvent {
        message: "Hello World".to_string(),
        at: 123
    });
}

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