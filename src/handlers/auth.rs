use crate::handlers::state;
use crate::handlers::utils;
use crate::services::auth;
use crate::services::jwt;

use axum::body::Body;
use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RegisterResponse {
    token: String,
}

impl RegisterResponse {
    fn new(token: String) -> Self {
        RegisterResponse { token }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterPayload {
    email: String,
    password: String,
}

impl RegisterPayload {
    fn validate(&self) -> Result<(), String> {
        if self.email.len() < 6 {
            return Err("length of email need be more than 6".to_string());
        }
        if self.password.len() < 6 {
            return Err("length of password need to be more than 6".to_string());
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
    match auth::register(&_state.db_pool, _payload.email, _payload.password).await {
        Ok(_token) => token = _token,
        Err(e) => {
            return (e.http_code, Json(utils::Response::new(e.message))).into_response();
        }
    }

    (StatusCode::CREATED, Json(RegisterResponse::new(token))).into_response()
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

impl LoginResponse {
    fn new(token: String) -> Self {
        LoginResponse { token }
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
            return Err("length of password need to be more than 6".to_string());
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
    match auth::login(&_state.db_pool, _payload.email, _payload.password).await {
        Ok(_token) => token = _token,
        Err(e) => {
            return (e.http_code, Json(utils::Response::new(e.message))).into_response();
        }
    }

    (StatusCode::OK, Json(LoginResponse::new(token))).into_response()
}

pub async fn validate(mut req: Request<Body>, next: Next) -> Response {
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
        Ok(current_user_id) => {
            req.extensions_mut().insert(current_user_id);
            next.run(req).await
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(utils::Response::new(e.message)),
        )
            .into_response(),
    }
}
