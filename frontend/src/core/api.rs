use reqwasm::http::Request;
use serde::de::DeserializeOwned;

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
