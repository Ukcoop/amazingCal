use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Time {
    pub year: i16,
    pub month: i16,
    pub day: i16,
    pub hour: i16,
    pub minute: i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub name: String,
    pub start: Time,
    pub end: Time,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Calendar {
    pub name: String,
    pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize)]
pub struct UserData {
    pub calendars: Vec<Calendar>,
}
