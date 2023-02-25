use crate::Event;
use candid::Principal;

pub fn notify_subscriber(
    canister: Principal,
    method: String,
    events: &Vec<Event>,
) -> Result<(), ()> {
    ic_cdk::api::call::notify(canister, &*method, (events,)).map_err(|e| ())
}
