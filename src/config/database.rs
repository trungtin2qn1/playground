use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub db_name: String,
    pub user: String,
    pub host: String,
    pub password: String,
}
