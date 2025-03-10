use once_cell::sync::Lazy;
use regex::Regex;
use web_sys::console;
use yew::{function_component, html, use_state, Callback, Html, MouseEvent, UseStateHandle};

use crate::components::main::{
    button::{Button, ButtonStyle},
    input_field::InputField,
    status::{Status, StatusCode, StatusObject},
};

static HAS_LOWERCASE: Lazy<Result<Regex, regex::Error>> = Lazy::new(|| Regex::new(r"[a-z]"));
static HAS_UPPERCASE: Lazy<Result<Regex, regex::Error>> = Lazy::new(|| Regex::new(r"[A-Z]"));
static HAS_DIGIT: Lazy<Result<Regex, regex::Error>> = Lazy::new(|| Regex::new(r"\d"));
static HAS_SYMBOL: Lazy<Result<Regex, regex::Error>> =
    Lazy::new(|| Regex::new(r#"[!@#$%^&*(),.?":{}|<>_\-+=~`\[\]\\;/]"#));

fn is_password_valid(password: &str) -> bool {
    password.len() >= 8
        && HAS_LOWERCASE.as_ref().is_ok_and(|r| r.is_match(password))
        && HAS_UPPERCASE.as_ref().is_ok_and(|r| r.is_match(password))
        && HAS_DIGIT.as_ref().is_ok_and(|r| r.is_match(password))
        && HAS_SYMBOL.as_ref().is_ok_and(|r| r.is_match(password))
}

fn is_valid_login(
    email: &UseStateHandle<String>,
    password: &UseStateHandle<String>,
    status: &UseStateHandle<StatusObject>,
) -> bool {
    if email.is_empty() || password.is_empty() {
        let error = format!(
            "The following fields are not filled out:\n{}{}",
            if email.is_empty() { "email\n" } else { "" },
            if password.is_empty() {
                "password\n"
            } else {
                ""
            },
        );

        status.set(StatusObject {
            code: StatusCode::Error,
            data: error,
        });
        return false;
    }

    if !is_password_valid(password) {
        status.set(StatusObject {
            code: StatusCode::Error,
            data: "Your password must contain:\n- One uppercase letter\n- One lowercase letter\n- One digit\n- One special character".to_string(),
        });
        return false;
    }

    true
}

#[function_component]
pub fn Login() -> Html {
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
                    data: "".to_string(),
                });
                console::log_1(&"Login button clicked!".into());
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
                    data: "".to_string(),
                });
                console::log_1(&"Signup button clicked!".into());
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
                    <Button text={"Log in"} style={ButtonStyle::Primary} width="w-full" on_click={handle_login} />
                    <Button text={"Sign up"} style={ButtonStyle::Primary} width="w-full" on_click={handle_signup} />
                </div>
            </div>
        </div>
    }
}
