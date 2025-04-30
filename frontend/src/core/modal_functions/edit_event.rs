use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{hook, use_state, UseStateHandle};

use crate::components::{
    main::status::{StatusCode, StatusObject},
    modal::time_editor::{States, StatesContainer},
};

use crate::core::{
    api::post,
    event_manager::EventDisplayManager,
    shared::{Event, Time},
};

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

    return post::<Event>("/api/update/event", &token, &new_event).await;
}

pub async fn delete_event(uuid: String, token: String) -> u16 {
    let delete_event = DeleteEvent { uuid };

    return post::<DeleteEvent>("/api/delete/event", &token, &delete_event).await;
}

pub fn handle_submit(
    name: String,
    uuid: String,
    states: StatesContainer,
    old_key: String,
    token: String,
    status: UseStateHandle<StatusObject>,
    modal: UseStateHandle<String>,
) {
    let mut display_manager = match EventDisplayManager::get_instance().lock() {
        Ok(manager) => manager,
        Err(_) => {
            return;
        }
    };

    status.set(StatusObject {
        code: StatusCode::Loading,
        data: "Editing event...".to_string(),
    });

    spawn_local(async move {
        let code = edit_event(
            name.clone(),
            uuid.clone(),
            states.start.clone(),
            states.end.clone(),
            token,
        )
        .await;

        if code == 200 {
            status.set(StatusObject {
                code: StatusCode::Success,
                data: "Event edited successfully".to_string(),
            });

            modal.set("None".to_string());

            web_sys::console::log_1(&format!("Editing event: {}", old_key).into());

            display_manager.edit_event(
                old_key,
                Event {
                    name,
                    uuid,
                    start: Time {
                        day: *states.start.day,
                        month: *states.start.month,
                        year: *states.start.year,
                        hour: *states.start.hour + (12 * *states.start.ampm),
                        minute: *states.start.minute,
                    },
                    end: Time {
                        day: *states.end.day,
                        month: *states.end.month,
                        year: *states.end.year,
                        hour: *states.end.hour + (12 * *states.end.ampm),
                        minute: *states.end.minute,
                    },
                },
            )
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
    old_key: String,
    status: UseStateHandle<StatusObject>,
    modal: UseStateHandle<String>,
    open: UseStateHandle<String>,
) {
    let mut display_manager = match EventDisplayManager::get_instance().lock() {
        Ok(manager) => manager,
        Err(_) => {
            return;
        }
    };

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

            display_manager.delete_event(old_key, uuid);
        } else {
            status.set(StatusObject {
                code: StatusCode::Error,
                data: format!("Error editing event: {}", code),
            });
        }
    });

    open.set("None".to_string());
}
