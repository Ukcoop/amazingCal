use actix_web::web::{Data, Json};
use actix_web::{post, HttpRequest};
use actix_web::{HttpResponse, Responder};

use serde::Deserialize;

use crate::core::security::validate_request::validate_request;
use crate::{AppState, ErrorResponse};

use crate::core::calendar::delete::event::delete_event;

#[derive(Deserialize)]
struct RequestData {
    uuid: String,
}

#[post("/api/delete/event")]
pub async fn api_delete_event(
    req: HttpRequest,
    data: Json<RequestData>,
    app_state: Data<AppState>,
) -> impl Responder {
    let (uuid, result) = validate_request(req, &app_state.jwt_secret).await;
    if uuid == *"" {
        return result;
    }

    return match delete_event(&data.uuid, &app_state.database).await {
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

    use crate::core::init_db::{CalendarTable, EventTable};
    use crate::services::database::Database;

    use crate::routes::get::user_data::tests::create_valid_token;

    use crate::core::calendar::{
        create::event::tests::get_database_with_filled_calendar, get::calendars::get_calendars,
        get::events::get_events, parse_calendar_data::parse_calendar, shared::Calendar,
    };

    #[actix_web::test]
    async fn test_api_delete_event() {
        let database: Database = get_database_with_filled_calendar().await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    jwt_secret: "my_secret".to_string(),
                    database: database.clone(),
                }))
                .service(api_delete_event),
        )
        .await;

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        let parsed_calendar: Calendar = match parse_calendar(&calendars_from_db[0], &database).await
        {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to get calendars. {}", e)
            }
        };

        let valid_token = create_valid_token("test_user", "my_secret");

        let test_data = json!({
            "uuid": parsed_calendar.events[0].uuid,
        });

        let reqest = test::TestRequest::post()
            .uri("/api/delete/event")
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

        assert_eq!(events_from_db, vec![]);
    }
}
