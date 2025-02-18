use crate::ErrorResponse;
use actix_web::{HttpRequest, HttpResponse};

use crate::core::security::jwt_authentication::is_valid_token;

pub async fn validate_request(req: HttpRequest, jwt_secret: &String) -> (String, HttpResponse) {
    let token: String = match req.headers().get("Authorization") {
        Some(result) => result.to_str().unwrap_or("").to_string(),
        None => "".to_string(),
    };

    if token == *"" {
        return (
            "".to_string(),
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Token was not found.".to_string(),
            }),
        );
    }

    let (valid, uuid) = is_valid_token(token, jwt_secret);

    if !valid {
        return (
            "".to_string(),
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Token is invalid.".to_string(),
            }),
        );
    }

    return (uuid, HttpResponse::Ok().into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header, test};
    use jsonwebtoken::{encode, EncodingKey, Header};

    use crate::core::security::jwt_authentication::Session;

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

    fn create_expired_token(sub: &str, secret: &str) -> String {
        let claims = Session {
            sub: sub.to_string(),
            exp: 0,
        };

        return encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("Failed to create token");
    }

    #[tokio::test]
    async fn test_invalid_token() {
        let req = test::TestRequest::get()
            .uri("/")
            .insert_header((header::AUTHORIZATION, "invalid.token"))
            .to_http_request();

        let (uuid, _) = validate_request(req, &"".to_string()).await;
        assert_eq!(uuid, "");
    }

    #[tokio::test]
    async fn test_expired_token() {
        let valid_token = create_expired_token("test_user", "my_secret");

        let req = test::TestRequest::get()
            .uri("/")
            .insert_header((header::AUTHORIZATION, valid_token))
            .to_http_request();

        let (uuid, _) = validate_request(req, &"my_secret".to_string()).await;
        assert_eq!(uuid, "");
    }

    #[tokio::test]
    async fn test_valid_token() {
        let valid_token = create_valid_token("test_user", "my_secret");

        let req = test::TestRequest::get()
            .uri("/")
            .insert_header((header::AUTHORIZATION, valid_token))
            .to_http_request();

        let (uuid, _) = validate_request(req, &"my_secret".to_string()).await;
        assert_eq!(uuid, "test_user");
    }
}
