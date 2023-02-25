pub use crate::event::{Event, SimpleEvent};
pub use crate::storage::EventState;

use candid::CandidType;
use ic_cdk::export::Principal;
use serde::Serialize;
use std::borrow::Cow;

mod event;
mod storage;
mod subscriber;


pub fn save() -> Cow<'static, [u8]> {
    storage::get_serialized_state()
}

pub fn restore(state: storage::EventState) {
    storage::set_event_state(state)
}

pub fn publish_event<T>(event: &T)
where
    T: CandidType + Serialize + SimpleEvent,
{
    storage::publish_event(event);
}

pub fn add_subscriber(sub: Principal) {
    storage::event_state_mut(|state| {
        state.subscribers.insert(sub);
    });
}

pub fn remove_subscriber(sub: &Principal) {
    storage::event_state_mut(|state| {
        state.subscribers.remove(sub);
    });
}

pub fn is_subscriber(sub: &Principal) -> bool {
    storage::event_state_mut(|state| state.subscribers.contains(sub))
}
