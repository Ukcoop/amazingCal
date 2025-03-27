use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub uuid: String,
    pub start: Time,
    pub end: Time,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug, Clone)]
pub struct Calendar {
    pub name: String,
    pub uuid: String,
    pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct UserData {
    pub calendars: Vec<Calendar>,
}
