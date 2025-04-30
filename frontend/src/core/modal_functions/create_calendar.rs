use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, UseStateHandle};

use crate::components::main::status::{StatusCode, StatusObject};

use crate::core::api::post;

#[derive(Serialize)]
struct CreateCalendarInput {
    name: String,
}

pub async fn create_calendar(name: String, token: String) -> u16 {
    let input = CreateCalendarInput { name };

    return post::<CreateCalendarInput>("/api/create/calendar", &token, &input).await;
}

pub fn handle_submit(
    name: String,
    token: String,
    modal: UseStateHandle<String>,
    status: UseStateHandle<StatusObject>,
    refresh: Callback<()>,
) {
    spawn_local(async move {
        let code = create_calendar(name, token).await;

        if code == 200 {
            status.set(StatusObject {
                code: StatusCode::Ok,
                data: "Event edited successfully".to_string(),
            });

            modal.set("None".to_string());
            refresh.emit(());
        } else {
            status.set(StatusObject {
                code: StatusCode::Error,
                data: format!("Error editing event: {}", code),
            });
        }
    });
}
