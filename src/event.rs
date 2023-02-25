use std::borrow::Cow;

use ic_cdk::export::candid::CandidType;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct Event {
    event_schema: String,
    event_type: String,
    created_at: u64,
    data: Vec<u8>,
}

impl Event {
    pub fn new(event_schema: String, event_type: String, created_at: u64, data: Vec<u8>) -> Self {
        Event {
            event_schema,
            event_type,
            created_at,
            data,
        }
    }

    pub fn into<T : CandidType + DeserializeOwned>(self) -> candid::Result<T> {
        candid::decode_one::<T>(&*self.data)
    }

    pub fn event_schema(&self) -> &str {
        &self.event_schema
    }
    pub fn event_type(&self) -> &str {
        &self.event_type
    }
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

pub trait SimpleEvent {
    fn event_schema(&self) -> String;
    fn event_type(&self) -> String;
    fn created_at(&self) -> u64;
}

impl Storable for Event {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::from(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&*bytes).unwrap()
    }
}
