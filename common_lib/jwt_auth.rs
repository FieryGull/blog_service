#![allow(dead_code, unused)]

use crate::common_lib::error_handler::CustomError;
use actix_web::dev::Payload;
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::{ready, Ready};
use chrono::Duration;
#[cfg(not(test))]
use chrono::Utc;
#[cfg(test)]
use crate::common_lib::utils::test_utils::Utc;


#[derive(Serialize, Deserialize)]
pub struct Token {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl Token {
    fn new(sub: String) -> Self {
        let now = Utc::now();
        Token {
            sub,
            iat: now.timestamp() as usize,
            exp: (now + Duration::minutes(60)).timestamp() as usize,
        }
    }
    pub fn create(sub: String, jwt_secret: String) -> Result<String, CustomError> {
        let claim = Token::new(sub);
        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
            .map_err(|_| CustomError::new(500, "Failed to create jwt token".to_string()))
    }

    pub fn verify(token: String, jwt_secret: String) -> Result<Token, CustomError> {
        let data = match decode::<Token>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => Ok(c.claims),
            Err(_) => { Err(CustomError::new(401, "Invalid token".to_string())) }
        };
        data
    }
}

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = CustomError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().to_string())
            });

        if token.is_none() {
            let json_error = CustomError {
                error_status_code: 401,
                error_message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(json_error));
        }

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found.");
        let claim = match Token::verify(token.unwrap(), jwt_secret) {
            Ok(token) => token,
            Err(err) => return ready(Err(err)),
        };

        let user_id = uuid::Uuid::parse_str(claim.sub.as_str()).unwrap();
        ready(Ok(JwtMiddleware { user_id }))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn succeed_token_claim_new() {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let claims = Token::new(id.clone());

        assert_eq!(claims.sub, id);
        assert_eq!(claims.iat, now.timestamp() as usize);
        assert_eq!(claims.exp, (now + Duration::minutes(60)).timestamp() as usize);
    }

    #[test]
    fn succeed_create_jwt_token() {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found.");
        let id = Uuid::new_v4().to_string();
        assert!(Token::create(id, jwt_secret).is_ok());
    }

    #[test]
    fn succeed_verify_jwt_token() {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found.");
        let id = Uuid::new_v4().to_string();
        let token = Token::create(id, jwt_secret.clone());

        assert!(Token::verify(token.unwrap(), jwt_secret).is_ok());
    }

    #[test]
    fn fail_verify_wrong_token() {
        assert!(Token::verify("wrong_token".to_string(), "secret".to_string()).is_err());
    }
}