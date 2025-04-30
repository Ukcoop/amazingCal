use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::UseStateHandle;
use yew_router::prelude::Navigator;

use crate::core::{
    event_manager::EventDisplayManager,
    shared::{Calendar, UserData},
};
use crate::Route;

use crate::core::api::get;

#[derive(PartialEq, Clone)]
pub struct ActiveCalendar {
    pub name: String,
    pub uuid: String,
}

#[wasm_bindgen(module = "/src/js/auth_handler.js")]
extern "C" {
    async fn get_session() -> JsValue;
}

pub fn get_current_session(navigator: Option<Navigator>, token: UseStateHandle<String>) {
    spawn_local(async move {
        let new_token = get_session().await.as_string().unwrap_or_default();

        if new_token.is_empty() {
            if let Some(navigator) = navigator {
                navigator.push(&Route::Login);
            }
        } else {
            token.set(new_token);
        }
    });
}

pub fn get_user_data(
    calendars: UseStateHandle<Vec<Calendar>>,
    active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    navigator: Option<Navigator>,
    token: UseStateHandle<String>,
) {
    if token.is_empty() {
        return;
    }

    spawn_local(async move {
        let (res, code) = get::<UserData>("/api/get/userData", &token).await;

        if code == 200 {
            calendars.set(res.calendars.clone());
            let mut calendars: Vec<ActiveCalendar> = Vec::new();

            for calendar in &res.calendars {
                calendars.push(ActiveCalendar {
                    name: calendar.name.clone(),
                    uuid: calendar.uuid.clone(),
                });
            }

            active_calendars.set(calendars);

            let mut display_manager = match EventDisplayManager::get_instance().lock() {
                Ok(manager) => manager,
                Err(_) => {
                    return;
                }
            };

            display_manager.clear_events();
            for calendar in &res.calendars {
                for event in &calendar.events {
                    display_manager.add_event(calendar.uuid.clone(), event.clone());
                    web_sys::console::log_1(&JsValue::from_str("added event"));
                }
            }
        } else if code == 400 || code == 401 {
            if let Some(navigator) = &navigator {
                navigator.push(&Route::Login);
            }
        } else {
            web_sys::console::log_1(&JsValue::from_str(&format!(
                "something went wrong, code: {}",
                code
            )));
        }
    });
}
