use crate::repositories::user;
use crate::{common::error, services::jwt};
use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn register(
    db_pool: &deadpool_postgres::Pool,
    email: String,
    password: String,
) -> Result<String, error::Error> {
    let hashed = hash(password, DEFAULT_COST)?;
    let id = user::create_user(db_pool, user::UserDb::new(0, email, hashed)).await?;
    let token = jwt::create_token(id)?;

    Ok(token)
}

pub async fn login(
    db_pool: &deadpool_postgres::Pool,
    email: String,
    password: String,
) -> Result<String, error::Error> {
    let user_db = user::get_user_by_email(db_pool, &email).await?;
    let user_db = if let Some(user_db) = user_db {
        user_db
    } else {
        return Err(error::Error {
            kind: "unauthorized".to_string(),
            message: "this email did not register".to_string(),
            http_code: StatusCode::UNAUTHORIZED,
        });
    };
    let valid = verify(password, &user_db.password)?;
    if !valid {
        return Err(error::Error {
            kind: "unauthorized".to_string(),
            message: "password is incorrect".to_string(),
            http_code: StatusCode::UNAUTHORIZED,
        });
    }
    let token = jwt::create_token(user_db.id)?;

    Ok(token)
}
