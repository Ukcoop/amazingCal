use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub name: String,
    pub uuid: String,
    pub start: Time,
    pub end: Time,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Calendar {
    pub name: String,
    pub uuid: String,
    pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize)]
pub struct UserData {
    pub calendars: Vec<Calendar>,
}
