use yew::{hook, use_state};

use crate::components::modal::time_editor::States;

use crate::core::{
    api::post,
    shared::{Event, Time},
};

#[hook]
pub fn use_get_states(time: Time) -> States {
    States {
        day: use_state(|| time.day),
        month: use_state(|| time.month),
        year: use_state(|| time.year),
        hour: use_state(|| {
            if time.hour > 12 {
                time.hour - 12
            } else {
                time.hour
            }
        }),
        minute: use_state(|| time.minute),
        ampm: use_state(|| if time.hour > 12 { 1 } else { 0 }),
    }
}

pub async fn edit_event(
    name: String,
    uuid: String,
    start: States,
    end: States,
    token: String,
) -> u16 {
    let new_event = Event {
        name,
        uuid,
        start: Time {
            day: *start.day,
            month: *start.month,
            year: *start.year,
            hour: *start.hour + (12 * *start.ampm),
            minute: *start.minute,
        },
        end: Time {
            day: *end.day,
            month: *end.month,
            year: *end.year,
            hour: *end.hour + (12 * *end.ampm),
            minute: *end.minute,
        },
    };

    return post::<Event>("http://localhost:3080/api/update/event", &token, &new_event).await;
}
