use axum::http::StatusCode;

use crate::{
    common::error,
    repositories::user::{self},
};

pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
}

impl User {
    pub fn new(id: i64, email: String, name: String) -> Self {
        User { id, email, name }
    }
}

pub async fn get_user_by_id(
    db_pool: &deadpool_postgres::Pool,
    id: &i64,
) -> Result<User, error::Error> {
    match user::get_user_by_id(db_pool, id).await? {
        Some(user_db) => Ok(User::new(*id, user_db.email, user_db.name)),
        None => Err(error::Error {
            kind: "unauthorized".to_string(),
            message: "this email did not register".to_string(),
            http_code: StatusCode::UNAUTHORIZED,
        }),
    }
}
