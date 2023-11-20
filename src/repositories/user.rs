use serde::{Deserialize, Serialize};

use crate::common::error;
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDb {
    pub email: String,
    pub password: String,
    pub name: String,
}

impl UserDb {
    pub fn new(email: String, password: String, name: String) -> Self {
        UserDb {
            email,
            password,
            name,
        }
    }
}

impl Default for UserDb {
    fn default() -> Self {
        UserDb {
            email: "".to_string(),
            password: "".to_string(),
            name: "".to_string(),
        }
    }
}

pub fn get_user_by_email(db: &sled::Db, email: &String) -> Result<Option<UserDb>, error::Error> {
    let value = db.get(email.as_bytes())?;
    match value {
        Some(value) => Ok(serde_json::from_slice(&value)?),
        None => Ok(None),
    }
}

pub fn create_user(db: &mut sled::Db, user_db: UserDb) -> Result<(), error::Error> {
    db.insert(user_db.email.clone(), json!(user_db).to_string().as_bytes())?;

    Ok(())
}
