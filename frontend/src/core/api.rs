use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Serialize};

pub async fn get<T: DeserializeOwned + Default>(path: &str, token: &str) -> (T, u16) {
    let base_url = option_env!("BASE_API_URL").unwrap_or("").to_string();

    let response = match Request::get(format!("{}{}", base_url, path).as_str())
        .header("Content-Type", "application/json")
        .header("Authorization", token)
        .send()
        .await
    {
        Ok(result) => result,
        Err(_) => return (T::default(), 0),
    };

    let body = (response.json().await).unwrap_or_default();
    return (body, response.status());
}

pub async fn post<T: Serialize>(path: &str, token: &str, json: &T) -> u16 {
    let base_url = option_env!("BASE_API_URL").unwrap_or("").to_string();

    let body_string = match serde_json::to_string(json) {
        Ok(s) => s,
        Err(_) => return 0,
    };

    web_sys::console::log_1(&format!("{}{}", base_url, path).into());

    let response = match Request::post(format!("{}{}", base_url, path).as_str())
        .header("Content-Type", "application/json")
        .header("Authorization", token)
        .body(body_string)
        .send()
        .await
    {
        Ok(result) => result,
        Err(_) => return 0,
    };

    return response.status();
}
