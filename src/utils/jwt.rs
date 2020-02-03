use crate::{models::user::User, TOKEN_KEY};
use chrono::prelude::*;
use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    iat: usize,
    sub: String,
    exp: usize,
    id: i32,
    username: String,
}

impl JWTClaims {
    pub fn encode(user: &User) -> String {
        let now: DateTime<Utc> = Utc::now();
        let expire: DateTime<Utc> = Utc::now().add(Duration::days(7));
        let claims = JWTClaims {
            iat: now.timestamp() as usize,
            sub: String::from("LOGIN_TOKEN"),
            exp: expire.timestamp() as usize,
            id: user.id,
            username: user.username.clone(),
        };

        jwt_encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&TOKEN_KEY),
        )
        .unwrap()
    }

    pub fn decode(token: String) -> Result<String, ()> {
        let claims = jwt_decode::<JWTClaims>(
            token.as_str(),
            &DecodingKey::from_secret(&TOKEN_KEY),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ())?;
        Ok(claims.claims.username)
    }
}
