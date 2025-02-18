use actix_web::web::Data;
use actix_web::{get, HttpRequest};
use actix_web::{HttpResponse, Responder};

use crate::{AppState, ErrorResponse};

use crate::core::calendar::get::user_data::get_user_data;
use crate::core::security::validate_request::validate_request;

#[get("/api/get/userData")]
pub async fn api_get_user_data(req: HttpRequest, app_state: Data<AppState>) -> impl Responder {
    let (uuid, result) = validate_request(req, &app_state.jwt_secret).await;
    if uuid == *"" {
        return result;
    }

    return match get_user_data(&uuid, &app_state.database).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::core::calendar::shared::UserData;
    use actix_web::{
        http::{self, header},
        test, App,
    };
    use jsonwebtoken::{encode, EncodingKey, Header};

    use crate::core::init_db::tests::get_testable_db;
    use crate::core::security::jwt_authentication::Session;
    use crate::services::database::Database;

    pub fn create_valid_token(sub: &str, secret: &str) -> String {
        let claims = Session {
            sub: sub.to_string(),
            exp: 10000000000,
        };

        return encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("Failed to create token");
    }

    #[actix_web::test]
    async fn test_api_get_user_data() {
        let database: Database = get_testable_db().await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    jwt_secret: "my_secret".to_string(),
                    database,
                }))
                .service(api_get_user_data),
        )
        .await;

        let valid_token = create_valid_token("test_user", "my_secret");

        let reqest = test::TestRequest::get()
            .uri("/api/get/userData")
            .insert_header((header::AUTHORIZATION, valid_token))
            .to_request();

        let response = test::call_service(&app, reqest).await;

        assert_eq!(response.status(), http::StatusCode::OK);

        let body: UserData = test::read_body_json(response).await;
        assert_eq!(body.calendars[0].name, "default");
    }
}
