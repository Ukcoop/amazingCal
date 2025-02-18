use actix_web::web::{Data, Json};
use actix_web::{post, HttpRequest};
use actix_web::{HttpResponse, Responder};

use serde::Deserialize;

use crate::core::security::validate_request::validate_request;
use crate::{AppState, ErrorResponse};

use crate::core::calendar::delete_calendar::delete_calendar;

#[derive(Deserialize)]
struct RequestData {
    uuid: String,
}

#[post("/api/delete/calendar")]
pub async fn api_delete_calendar (
    req: HttpRequest,
    data: Json<RequestData>,
    app_state: Data<AppState>,
) -> impl Responder {
    let (uuid, result) = validate_request(req, &app_state.jwt_secret).await;
    if uuid == *"" {
        return result;
    }

    return match delete_calendar(&data.uuid, &app_state.database).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use actix_web::{http, http::header, test, App};
    use serde_json::json;

    use crate::core::init_db::CalendarTable;
    use crate::services::database::Database;

    use crate::routes::get::user_data::tests::create_valid_token;

    use crate::core::calendar::{ create_event::tests::get_database_with_filled_calendar, get_calendars::get_calendars };

    #[actix_web::test]
    async fn test_api_delete_calendar() {
        let database: Database = get_database_with_filled_calendar().await;
       
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    jwt_secret: "my_secret".to_string(),
                    database: database.clone(),
                }))
                .service(api_delete_calendar),
        )
        .await;

        let mut calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        let valid_token = create_valid_token("test_user", "my_secret");

        let test_data = json!({
            "uuid": calendars_from_db[0].uuid,
        });

        let reqest = test::TestRequest::post()
            .uri("/api/delete/calendar")
            .set_json(&test_data)
            .insert_header((header::AUTHORIZATION, valid_token))
            .to_request();

        let response = test::call_service(&app, reqest).await;
        assert_eq!(response.status(), http::StatusCode::OK);

        calendars_from_db =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        assert_eq!(calendars_from_db, vec![]);
    } 
}
