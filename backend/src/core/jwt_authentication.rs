use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub sub: String,
    pub exp: u64,
}

pub fn is_valid_token(token: String, jwt_secret: &String) -> (bool, String) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, Header, EncodingKey};

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
    async fn test_valid_token() {
        let secret = "my_secret";
        let sub = "test_user";
        let token = create_valid_token(sub, secret);
        
        let (is_valid, decoded_sub) = is_valid_token(token, &secret.to_string());

        assert!(is_valid);
        assert_eq!(decoded_sub, sub);
    }

    #[actix_web::test]
    async fn test_invalid_token() {
        let secret = "my_secret";
        let invalid_token = "invalid.token.string";
        
        let (is_valid, decoded_sub) = is_valid_token(invalid_token.to_string(), &secret.to_string());

        assert!(!is_valid);
        assert_eq!(decoded_sub, "");
    }

    #[actix_web::test]
    async fn test_empty_token() {
        let secret = "my_secret";
        
        let (is_valid, decoded_sub) = is_valid_token("".to_string(), &secret.to_string());

        assert!(!is_valid);
        assert_eq!(decoded_sub, "");
    }

    #[actix_web::test]
    async fn test_empty_secret() {
        let sub = "test_user";
        let token = create_valid_token(sub, "my_secret");

        let (is_valid, decoded_sub) = is_valid_token(token, &"".to_string());

        assert!(!is_valid);
        assert_eq!(decoded_sub, "");
    }
}
