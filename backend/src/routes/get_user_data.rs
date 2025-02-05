use actix_web::web::Data;
use actix_web::{get, HttpRequest};
use actix_web::{HttpResponse, Responder};

use crate::core::jwt_authentication::is_valid_token;
use crate::{AppState, ErrorResponse};

use crate::core::calendar::get_user_data::get_user_data;

#[get("/api/getUserData")]
pub async fn api_get_user_data(req: HttpRequest, app_state: Data<AppState>) -> impl Responder {
    let token: String = match req.headers().get("Authorization") {
        Some(result) => result.to_str().unwrap_or("").to_string(),
        None => "".to_string(),
    };

    if token == *"" {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Token was not found.".to_string(),
        });
    }

    let (valid, uuid) = is_valid_token(token, &app_state.jwt_secret);

    if !valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Token is invalid.".to_string(),
        });
    }

    return match get_user_data(&uuid, &app_state.database).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::calendar::shared::UserData;
    use actix_web::{
        http::{self, header},
        test, App,
    };
    use jsonwebtoken::{encode, EncodingKey, Header};

    use crate::core::{init_db::init_db, jwt_authentication::Session};
    use crate::services::database::{convert_sqlx_error, Database};

    fn create_valid_token(sub: &str, secret: &str) -> String {
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
    async fn test_no_token() {
        let database = match convert_sqlx_error(Database::new_db(true, "".to_string()).await) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };

        match init_db(&database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        }

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    jwt_secret: "my_secret".to_string(),
                    database,
                }))
                .service(api_get_user_data),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/getUserData")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_malformed_token() {
        let database = match convert_sqlx_error(Database::new_db(true, "".to_string()).await) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };

        match init_db(&database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        }

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    jwt_secret: "my_secret".to_string(),
                    database,
                }))
                .service(api_get_user_data),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/getUserData")
            .insert_header((header::AUTHORIZATION, "\"invalid.token\""))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_invalid_token() {
        let database = match convert_sqlx_error(Database::new_db(true, "".to_string()).await) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };

        match init_db(&database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        }

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    jwt_secret: "my_secret".to_string(),
                    database,
                }))
                .service(api_get_user_data),
        )
        .await;

        let invalid_token = "invalid_token";

        let req = test::TestRequest::get()
            .uri("/api/getUserData")
            .insert_header((header::AUTHORIZATION, invalid_token))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_valid_token() {
        let database = match convert_sqlx_error(Database::new_db(true, "".to_string()).await) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };

        match init_db(&database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        }

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

        let req = test::TestRequest::get()
            .uri("/api/getUserData")
            .insert_header((header::AUTHORIZATION, valid_token))
            .to_request();

        let resp = test::call_service(&app, req).await;
        println!("{:#?}", resp);

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: UserData = test::read_body_json(resp).await;
        assert_eq!(body.calendars[0].name, "default");
    }
}
