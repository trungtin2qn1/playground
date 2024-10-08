use crate::{handlers::state, handlers::utils, services::user};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct GetUserSelfResponse {
    id: i64,
    email: String,
}

impl GetUserSelfResponse {
    fn new(id: i64, email: String) -> Self {
        GetUserSelfResponse { id, email }
    }
}

pub async fn get_user_self(
    State(mut _state): State<state::RootState>,
    Extension(current_user_id): Extension<i64>,
) -> impl IntoResponse {
    let user = match user::get_user_by_id(&_state.db_pool, &current_user_id).await {
        Ok(user) => user,
        Err(e) => {
            return (e.http_code, Json(utils::Response::new(e.message))).into_response();
        }
    };

    (
        StatusCode::OK,
        Json(GetUserSelfResponse::new(user.id, user.email)),
    )
        .into_response()
}
