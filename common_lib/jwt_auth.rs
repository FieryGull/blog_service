#![allow(dead_code, unused)]

use crate::common_lib::error_handler::CustomError;
use actix_web::dev::Payload;
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::{ready, Ready};

pub fn create_jwt_token(jwt_secret: String, sub: String) -> String {
    let claims = TokenClaims::new(sub);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl TokenClaims {
    pub fn new(sub: String) -> Self {
        let now = Utc::now();
        TokenClaims {
            sub,
            iat: now.timestamp() as usize,
            exp: (now + Duration::minutes(60)).timestamp() as usize,
        }
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
        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = CustomError {
                    error_status_code: 401,
                    error_message: "Invalid token".to_string(),
                };
                return ready(Err(json_error));
            }
        };

        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}
