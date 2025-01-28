use actix_web::{get, HttpRequest};
use actix_web::{HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use crate::ErrorResponse;

use crate::core::jwt_authentication::is_valid_token;

#[derive(Deserialize)]
struct Token {
    token: String,
}

#[derive(Serialize)]
struct TestResponse {
    hello: String,
}

#[get("/api/getUserData")]
pub async fn get_user_data(req: HttpRequest) -> impl Responder {
    let token: String = match req.headers().get("Authorization") {
        Some(result) => result.to_str().unwrap_or("").to_string(),
        None => "".to_string(),
    };

    if token == *"" {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Token was not found.".to_string(),
        });
    }

    let token_data: Token = match serde_json::from_str(&token) {
        Ok(result) => result,
        Err(_) => Token {
            token: "".to_string(),
        },
    };

    if token_data.token == *"" {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Token is invalid.".to_string(),
        });
    }

    let (valid, _) = is_valid_token(token_data.token);

    if !valid {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Token is invalid.".to_string(),
        });
    }

    return HttpResponse::Ok().json(TestResponse {
        hello: "world".to_string(),
    });
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_user_data() {
        // test will go here
    }
}
*/
