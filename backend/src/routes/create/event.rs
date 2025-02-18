use actix_web::web::{Data, Json};
use actix_web::{post, HttpRequest};
use actix_web::{HttpResponse, Responder};

use serde::Deserialize;

use crate::core::security::validate_request::validate_request;
use crate::{AppState, ErrorResponse};

use crate::core::calendar::create::event::create_event;

use crate::core::calendar::shared::Time;

#[derive(Deserialize)]
struct RequestData {
    calendar_id: String,
    name: String,
    start: Time,
    end: Time,
}

#[post("/api/create/event")]
pub async fn api_create_event(
    req: HttpRequest,
    data: Json<RequestData>,
    app_state: Data<AppState>,
) -> impl Responder {
    let (uuid, result) = validate_request(req, &app_state.jwt_secret).await;
    if uuid == *"" {
        return result;
    }

    return match create_event(
        &data.calendar_id,
        &data.name,
        &data.start,
        &data.end,
        &app_state.database,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use actix_web::{http, http::header, test, App};
    use serde_json::json;

    use crate::core::calendar::create::event::tests::get_database_with_filled_calendar;
    use crate::core::init_db::{CalendarTable, EventTable};
    use crate::services::database::Database;

    use crate::routes::get::user_data::tests::create_valid_token;

    use crate::core::calendar::{get::calendars::get_calendars, get::events::get_events};

    #[actix_web::test]
    async fn test_api_create_event() {
        let database: Database = get_database_with_filled_calendar().await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    jwt_secret: "my_secret".to_string(),
                    database: database.clone(),
                }))
                .service(api_create_event),
        )
        .await;

        let valid_token = create_valid_token("test_user", "my_secret");

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        let test_data = json!({
            "calendar_id": calendars_from_db[0].uuid,
            "name": "Christmas",
            "start": {
                "year": 2025,
                "month": 12,
                "day": 25,
                "hour": 0,
                "minute": 0
            },
            "end": {
                "year": 2025,
                "month": 12,
                "day": 26,
                "hour": 0,
                "minute": 0
            }
        });

        let reqest = test::TestRequest::post()
            .uri("/api/create/event")
            .set_json(&test_data)
            .insert_header((header::AUTHORIZATION, valid_token))
            .to_request();

        let response = test::call_service(&app, reqest).await;
        assert_eq!(response.status(), http::StatusCode::OK);

        let events_from_db: Vec<EventTable> =
            match get_events(&calendars_from_db[0].uuid, &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        assert_eq!(events_from_db[1].name, "Christmas");
    }
}
