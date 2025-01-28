use dotenv::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    sub: String,
    exp: u64,
}

pub fn is_valid_token(token: String) -> (bool, String) {
    dotenv().ok();

    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(result) => result,
        Err(_) => "".to_string(),
    };

    if token.is_empty() || jwt_secret.is_empty() {
        return (false, "".to_string());
    }

    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_aud = false;

    return match decode::<Session>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    ) {
        Ok(token_data) => (true, token_data.claims.sub),
        Err(_) => (false, "".to_string()),
    };
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_is_valid_token() {
        // test will go here
    }
}
*/
