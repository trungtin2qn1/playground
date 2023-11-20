use crate::{handlers::state, handlers::utils, services::user};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct GetUserSelfResponse {
    email: String,
    name: String,
}

impl GetUserSelfResponse {
    fn new(email: String, name: String) -> Self {
        GetUserSelfResponse { email, name }
    }
}

pub async fn get_user_self(
    State(mut _state): State<state::RootState>,
    Extension(current_user_email): Extension<String>,
) -> impl IntoResponse {
    let user = match user::get_user_by_email(&_state.db, &current_user_email) {
        Ok(user) => user,
        Err(e) => {
            return (e.http_code, Json(utils::Response::new(e.message))).into_response();
        }
    };

    (
        StatusCode::OK,
        Json(GetUserSelfResponse::new(user.email, user.name)),
    )
        .into_response()
}
