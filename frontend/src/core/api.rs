use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Serialize};

pub async fn get<T: DeserializeOwned + Default>(url: &str, token: &str) -> (T, u16) {
    let response = match Request::get(url)
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

pub async fn post<T: Serialize>(url: &str, token: &str, json: &T) -> u16 {
    let body_string = match serde_json::to_string(json) {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let response = match Request::post(url)
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
