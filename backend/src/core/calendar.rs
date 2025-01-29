use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Time {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub name: String,
    pub start: Time,
    pub end: Time
}

#[derive(Deserialize, Serialize)]
pub struct Calendar {
    pub name: String,
    pub events: Vec<Event>
}

#[derive(Deserialize, Serialize)]
pub struct UserData {
    pub calendars: Vec<Calendar>,
}

pub fn get_user_data(uuid: String) -> UserData {
    return UserData {
        calendars: vec![
            Calendar {
                name: "default".to_string(),
                events: vec![
                    Event {
                        name: "New year's eve".to_string(),
                        start: Time {
                            year: 2025,
                            month: 12,
                            day: 31,
                            hour: 0,
                            minute: 0
                        },
                        end: Time {
                            year: 2026,
                            month: 1,
                            day: 1,
                            hour: 0,
                            minute: 0
                        }
                    }
                ]
            }
        ]
    }
}
