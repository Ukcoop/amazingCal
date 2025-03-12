use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::UseStateHandle;
use yew_router::prelude::Navigator;

use crate::core::shared::{Calendar, UserData};
use crate::Route;

use crate::core::api::get;

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
    navigator: Option<Navigator>,
    token: UseStateHandle<String>,
) {
    if token.is_empty() {
        return;
    }

    spawn_local(async move {
        let (res, code) = get::<UserData>("http://localhost:3080/api/get/userData", &token).await;
        if code == 200 {
            calendars.set(res.calendars);
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
