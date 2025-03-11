use once_cell::sync::Lazy;
use regex::Regex;

use yew::UseStateHandle;

use crate::components::main::status::{StatusCode, StatusObject};

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

pub fn is_valid_login(
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
