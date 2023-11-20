use crate::repositories::user;
use crate::{common::error, services::jwt};
use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn register(
    db: &mut sled::Db,
    email: &String,
    password: &String,
    name: &String,
) -> Result<String, error::Error> {
    let user_db = user::get_user_by_email(db, email)?;
    if let Some(_) = user_db {
        return Err(error::Error {
            kind: "unauthorized".to_string(),
            message: "this email has been registered".to_string(),
            http_code: StatusCode::UNAUTHORIZED,
        });
    };
    let hashed = hash(password, DEFAULT_COST)?;
    let token = jwt::create_token(email.clone())?;

    user::create_user(
        db,
        user::UserDb::new(email.to_string(), hashed, name.to_string()),
    )?;

    Ok(token)
}

pub fn login(db: &mut sled::Db, email: &String, password: &String) -> Result<String, error::Error> {
    let user_db = user::get_user_by_email(db, email)?;
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
    let token = jwt::create_token(email.clone())?;

    Ok(token)
}
