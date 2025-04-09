use crate::core::api::post;
use serde::Serialize;

#[derive(Serialize)]
struct DeleteCalendar {
    uuid: String,
}

pub async fn delete_calendar(calendar_id: String, token: String) -> u16 {
    let delete_calendar = DeleteCalendar { uuid: calendar_id };

    return post::<DeleteCalendar>(
        "http://localhost:3080/api/delete/calendar",
        &token,
        &delete_calendar,
    )
    .await;
}
