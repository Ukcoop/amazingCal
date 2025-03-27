use once_cell::sync::Lazy;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;

use yew::{function_component, html, use_state, Callback, Html, MouseEvent, UseStateHandle};
use yew_router::hooks::use_navigator;

use crate::Route;

use crate::components::main::{
    button::{Button, ButtonStyle},
    input_field::InputField,
    status::{Status, StatusCode, StatusObject},
};

use crate::core::auth::is_valid_login;

static PUBLIC_SUPABASE_URL: Lazy<&str> =
    Lazy::new(|| option_env!("PUBLIC_SUPABASE_URL").unwrap_or(""));
static PUBLIC_ANON_KEY: Lazy<&str> = Lazy::new(|| option_env!("PUBLIC_ANON_KEY").unwrap_or(""));

#[wasm_bindgen(module = "/src/js/auth_handler.js")]
extern "C" {
    pub fn init_supabase(supabase_url: String, anon_key: String);
    pub async fn handle_login(email: String, password: String) -> JsValue;
    pub async fn handle_signup(email: String, password: String) -> JsValue;
}

async fn process_signup(email: String, password: String, status: UseStateHandle<StatusObject>) {
    let error = handle_signup(email, password).await;
    let parsed_error = error
        .as_string()
        .unwrap_or_else(|| "Something went wrong, please try again.".to_string());

    if parsed_error != "null" {
        status.set(StatusObject {
            code: StatusCode::Error,
            data: parsed_error,
        });
    } else {
        status.set(StatusObject {
            code: StatusCode::Success,
            data: "Signup successful, check your inbox to verify your account.".to_string(),
        });
    }
}

async fn process_login(
    email: String,
    password: String,
    status: UseStateHandle<StatusObject>,
) -> bool {
    let error = handle_login(email, password).await;
    let parsed_error = error
        .as_string()
        .unwrap_or_else(|| "Something went wrong, please try again.".to_string());

    if parsed_error != "null" {
        status.set(StatusObject {
            code: StatusCode::Error,
            data: parsed_error,
        });

        return false;
    } else {
        status.set(StatusObject {
            code: StatusCode::Ok,
            data: "".to_string(),
        });

        return true;
    }
}

#[function_component]
pub fn Login() -> Html {
    let navigator = use_navigator();
    init_supabase(PUBLIC_SUPABASE_URL.to_string(), PUBLIC_ANON_KEY.to_string());

    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let status = use_state(|| StatusObject {
        code: StatusCode::Ok,
        data: "".to_string(),
    });

    let handle_login = {
        let email = email.clone();
        let password = password.clone();
        let status = status.clone();
        Callback::from(move |_event: MouseEvent| {
            if is_valid_login(&email, &password, &status) {
                status.set(StatusObject {
                    code: StatusCode::Loading,
                    data: "Logging the user in...".to_string(),
                });

                let email = email.clone();
                let password = password.clone();
                let status = status.clone();
                let navigator = navigator.clone();
                spawn_local(async move {
                    let sucessful =
                        process_login(email.to_string(), password.to_string(), status.clone())
                            .await;

                    if sucessful {
                        if let Some(navigator) = &navigator {
                            navigator.push(&Route::Calendar);
                        }
                    }
                });
            }
        })
    };

    let handle_signup = {
        let email = email.clone();
        let password = password.clone();
        let status = status.clone();
        Callback::from(move |_event: MouseEvent| {
            if is_valid_login(&email, &password, &status) {
                status.set(StatusObject {
                    code: StatusCode::Loading,
                    data: "Signing user up...".to_string(),
                });

                spawn_local(process_signup(
                    email.to_string(),
                    password.to_string(),
                    status.clone(),
                ));
            }
        })
    };

    html! {
        <div class="flex flex-col p-5 h-screen max-h-screen items-center justify-center bg-white dark:bg-gray-950">
            <div class="flex flex-col max-w-xs">
                <label class="text-2xl pb-1">{"Email"}</label>
                <InputField<String> varient="email" value={email.clone()} />
                <label class="text-2xl pb-1">{"Password"}</label>
                <InputField<String> varient="password" value={password.clone()} />
                <Status status={status.clone()} />
                <div class="flex flex-col">
                    <Button style={ButtonStyle::Primary} width="w-full" on_click={handle_login}>{"Log in"}</Button>
                    <Button style={ButtonStyle::Primary} width="w-full" on_click={handle_signup}>{"Sign up"}</Button>
                </div>
            </div>
        </div>
    }
}
