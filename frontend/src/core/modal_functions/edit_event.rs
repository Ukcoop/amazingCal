use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{hook, use_state, Callback, UseStateHandle};

use crate::components::main::status::{StatusCode, StatusObject};
use crate::components::modal::time_editor::States;

use crate::core::{
    api::post,
    shared::{Event, Time},
};

#[derive(Serialize)]
struct CreateEvent {
    calendar_id: String,
    name: String,
    start: Time,
    end: Time,
}

#[derive(Serialize)]
struct DeleteEvent {
    uuid: String,
}

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

pub async fn create_event(
    name: String,
    start: States,
    end: States,
    calendar_id: String,
    token: String,
) -> u16 {
    let new_event = CreateEvent {
        calendar_id,
        name,
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

    return post::<CreateEvent>("http://localhost:3080/api/create/event", &token, &new_event).await;
}

pub async fn delete_event(uuid: String, token: String) -> u16 {
    let delete_event = DeleteEvent { uuid };

    return post::<DeleteEvent>(
        "http://localhost:3080/api/delete/event",
        &token,
        &delete_event,
    )
    .await;
}

pub fn handle_submit(
    name: String,
    uuid: String,
    start_states: States,
    end_states: States,
    token: String,
    status: UseStateHandle<StatusObject>,
    modal: UseStateHandle<String>,
    refresh_data: Callback<()>,
) {
    status.set(StatusObject {
        code: StatusCode::Loading,
        data: "Editing event...".to_string(),
    });

    spawn_local(async move {
        let code = edit_event(name, uuid, start_states.clone(), end_states.clone(), token).await;

        if code == 200 {
            status.set(StatusObject {
                code: StatusCode::Success,
                data: "Event edited successfully".to_string(),
            });

            modal.set("None".to_string());
            refresh_data.emit(());
        } else {
            status.set(StatusObject {
                code: StatusCode::Error,
                data: format!("Error editing event: {}", code),
            });
        }
    });
}

pub fn handle_delete(
    uuid: String,
    token: String,
    status: UseStateHandle<StatusObject>,
    modal: UseStateHandle<String>,
    open: UseStateHandle<String>,
    refresh_data: Callback<()>,
) {
    status.set(StatusObject {
        code: StatusCode::Loading,
        data: "Editing event...".to_string(),
    });

    spawn_local(async move {
        let code = delete_event(uuid.to_string(), token.to_string()).await;

        if code == 200 {
            status.set(StatusObject {
                code: StatusCode::Success,
                data: "Event edited successfully".to_string(),
            });

            modal.set("None".to_string());
            refresh_data.emit(());
        } else {
            status.set(StatusObject {
                code: StatusCode::Error,
                data: format!("Error editing event: {}", code),
            });
        }
    });

    open.set("None".to_string());
}
