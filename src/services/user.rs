use axum::http::StatusCode;

use crate::{
    common::error,
    repositories::user::{self},
};

pub struct User {
    pub email: String,
    pub name: String,
}

impl User {
    pub fn new(email: String, name: String) -> Self {
        User { email, name }
    }
}

pub fn get_user_by_email(db: &sled::Db, email: &String) -> Result<User, error::Error> {
    match user::get_user_by_email(db, email)? {
        Some(user_db) => Ok(User::new(user_db.email, user_db.name)),
        None => Err(error::Error {
            kind: "unauthorized".to_string(),
            message: "this email did not register".to_string(),
            http_code: StatusCode::UNAUTHORIZED,
        }),
    }
}
