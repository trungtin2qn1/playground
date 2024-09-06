use crate::common::error;
use axum::http::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimData {
    id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    created_at: usize,
    data: ClaimData,
}

pub fn create_token(id: i64) -> Result<String, error::Error> {
    let key = b"secret_key";

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;

    let claims = Claims {
        created_at: now.as_secs() as usize,
        exp: (now + Duration::new(10000000, 0)).as_secs() as usize,
        data: ClaimData { id },
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(key)) {
        Ok(token) => Ok(token),
        Err(e) => Err(error::Error {
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
            kind: "jwt".to_string(),
            message: e.to_string(),
        }),
    }
}

pub fn validate_token(token: String) -> Result<i64, error::Error> {
    let key = b"secret_key";
    let validation = Validation::default();

    match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => Ok(c.claims.data.id),
        Err(e) => Err(e.into()),
    }
}
