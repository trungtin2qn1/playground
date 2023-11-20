use crate::handlers::state;
use crate::handlers::utils;
use crate::services::auth;
use crate::services::jwt;

use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RegisterResponse {
    access_token: String,
}

impl RegisterResponse {
    fn new(access_token: String) -> Self {
        RegisterResponse { access_token }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterPayload {
    email: String,
    name: String,
    password: String,
}

impl RegisterPayload {
    fn validate(&self) -> Result<(), String> {
        if self.email.len() < 6 {
            return Err("length of email need be more than 6".to_string());
        }
        if self.password.len() < 6 {
            return Err("password of email need to be more than 6".to_string());
        }

        Ok(())
    }
}

pub async fn register(
    State(mut _state): State<state::RootState>,
    Json(_payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    if let Err(e) = _payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(utils::Response::new(e.to_string())),
        )
            .into_response();
    }

    let token;
    match auth::register(
        &mut _state.db,
        &_payload.email,
        &_payload.password,
        &_payload.name,
    ) {
        Ok(_token) => token = _token,
        Err(e) => {
            return (e.http_code, Json(utils::Response::new(e.message))).into_response();
        }
    }

    (StatusCode::CREATED, Json(RegisterResponse::new(token))).into_response()
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
}

impl LoginResponse {
    fn new(access_token: String) -> Self {
        LoginResponse { access_token }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginPayload {
    email: String,
    password: String,
}

impl LoginPayload {
    fn validate(&self) -> Result<(), String> {
        if self.email.len() < 6 {
            return Err("length of email need be more than 6".to_string());
        }
        if self.password.len() < 6 {
            return Err("password of email need to be more than 6".to_string());
        }

        Ok(())
    }
}

pub async fn login(
    State(mut _state): State<state::RootState>,
    Json(_payload): Json<LoginPayload>,
) -> impl IntoResponse {
    if let Err(e) = _payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(utils::Response::new(e.to_string())),
        )
            .into_response();
    }

    let token;
    match auth::login(&mut _state.db, &_payload.email, &_payload.password) {
        Ok(_token) => token = _token,
        Err(e) => {
            return (e.http_code, Json(utils::Response::new(e.message))).into_response();
        }
    }

    (StatusCode::OK, Json(LoginResponse::new(token))).into_response()
}

pub async fn validate<B>(mut req: Request<B>, next: Next<B>) -> Response {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(utils::Response::new("missing header".to_string())),
        )
            .into_response();
    };

    match jwt::validate_token(auth_header.to_string()) {
        Ok(current_user_email) => {
            req.extensions_mut().insert(current_user_email);
            next.run(req).await
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(utils::Response::new(e.message)),
        )
            .into_response(),
    }
}
