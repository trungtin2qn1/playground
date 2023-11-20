use axum::http::StatusCode;
use bcrypt;
use std::time::SystemTimeError;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum ErrorKind {
    SystemTime,
}

#[derive(Debug)]
pub struct Error {
    pub kind: String,
    pub message: String,
    pub http_code: axum::http::StatusCode,
}

impl From<SystemTimeError> for Error {
    fn from(e: SystemTimeError) -> Self {
        Error {
            kind: String::from("system_time"),
            message: e.to_string(),
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Error {
            kind: String::from("sled_db"),
            message: e.to_string(),
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error {
            kind: String::from("serde_json"),
            message: e.to_string(),
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(e: bcrypt::BcryptError) -> Self {
        Error {
            kind: String::from("bcrypt"),
            message: e.to_string(),
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Error {
            kind: String::from("jwt"),
            message: e.to_string(),
            http_code: StatusCode::UNAUTHORIZED,
        }
    }
}
