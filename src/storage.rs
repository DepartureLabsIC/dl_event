use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeSet;

use candid::{CandidType, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};

use crate::{Event, SimpleEvent};

thread_local! {
    static STATE : RefCell<EventState> = RefCell::new(EventState::default());
}

#[derive(Serialize, Deserialize, Default)]
pub struct EventState {
    pub subscribers: BTreeSet<Principal>,
}

impl Storable for EventState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::from(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&*bytes).unwrap()
    }
}

pub fn event_state<R>(f: impl FnOnce(&EventState) -> R) -> R {
    STATE.with(|data| f(&data.borrow()))
}

pub fn event_state_mut<R>(f: impl FnOnce(&mut EventState) -> R) -> R {
    STATE.with(|data| f(&mut data.borrow_mut()))
}

pub fn set_event_state(state: EventState) {
    STATE.with(|existing| existing.swap(&RefCell::new(state)))
}

// We're copying the data here twice which isn't great like at all. But, this should be tiny
pub fn get_serialized_state() -> Cow<'static, [u8]> {
    let bytes = STATE.with(|data| {
        let data = data.borrow();
        let bytes = data.to_bytes();
        return bytes.into_owned();
    });
    return Cow::from(bytes);
}

pub fn publish_event<T>(event: &T)
where
    T: CandidType + Serialize + SimpleEvent,
{
    let bytes = candid::encode_one(event).unwrap();
    let event = Event::new(
        event.event_schema(),
        event.event_type(),
        event.created_at(),
        bytes,
    );

    STATE.with(|data| {
        let data = data.borrow();
        for sub in &data.subscribers {
            match ic_cdk::notify(sub.clone(), "dl_events_simple_notify_v0", (event.clone(),)) {
                Ok(_) => {}
                Err(e) => {
                    ic_cdk::println!("Failed to notify subscriber {} error {:?}", sub.to_text(), e)
                }
            }
        }
    })
}
