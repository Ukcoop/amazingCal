use actix_web::{get, HttpRequest};
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use crate::{AppState, ErrorResponse};
use crate::core::jwt_authentication::is_valid_token;

#[derive(Deserialize, Serialize)]
struct TestResponse {
    hello: String,
}

#[get("/api/getUserData")]
pub async fn get_user_data(req: HttpRequest, app_state: Data<AppState>) -> impl Responder {
    let token: String = match req.headers().get("Authorization") {
        Some(result) => result.to_str().unwrap_or("").to_string(),
        None => "".to_string(),
    };

    if token == *"" {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Token was not found.".to_string(),
        });
    }

    let (valid, _) = is_valid_token(token, &app_state.jwt_secret);

    if !valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Token is invalid.".to_string(),
        });
    }

    return HttpResponse::Ok().json(TestResponse {
        hello: "world".to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::{self, header}, test, App};
    use jsonwebtoken::{encode, Header, EncodingKey};

    use crate::core::jwt_authentication::Session;

    fn create_valid_token(sub: &str, secret: &str) -> String {
        let claims = Session {
            sub: sub.to_string(),
            exp: 10000000000,
        };

        return encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ).expect("Failed to create token");
    }

    #[actix_web::test]
    async fn test_no_token() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState { jwt_secret: "my_secret".to_string() }))
                .service(get_user_data),
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
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState { jwt_secret: "my_secret".to_string() }))
                .service(get_user_data),
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
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState { jwt_secret: "my_secret".to_string() }))
                .service(get_user_data),
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
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState { jwt_secret: "my_secret".to_string() }))
                .service(get_user_data),
        )
        .await;

        let valid_token = create_valid_token("test_user", "my_secret");

        let req = test::TestRequest::get()
            .uri("/api/getUserData")
            .insert_header((header::AUTHORIZATION, valid_token))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: TestResponse = test::read_body_json(resp).await;
        assert_eq!(body.hello, "world");
    }
}
