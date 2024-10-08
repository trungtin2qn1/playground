use crate::common::error;
use deadpool_postgres::GenericClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDb {
    pub id: i64,
    pub email: String,
    pub password: String,
}

impl UserDb {
    pub fn new(id: i64, email: String, password: String) -> Self {
        UserDb {
            id,
            email,
            password,
        }
    }
}

impl Default for UserDb {
    fn default() -> Self {
        UserDb {
            id: 0,
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

pub async fn get_user_by_id(
    db_pool: &deadpool_postgres::Pool,
    id: &i64,
) -> Result<Option<UserDb>, error::Error> {
    let client = db_pool.get().await?;

    // Query the database for the user with the specified email
    let row = client
        .query_one("SELECT id, email FROM users WHERE id = $1", &[&id])
        .await?;

    let mut user = UserDb::default();
    user.id = row.get(0);
    user.email = row.get(1);

    Ok(Some(user))
}

pub async fn get_user_by_email(
    db_pool: &deadpool_postgres::Pool,
    email: &str,
) -> Result<Option<UserDb>, error::Error> {
    let client = db_pool.get().await?;

    // Query the database for the user with the specified email
    let row_opt = client
        .query_opt(
            "SELECT id, email, password FROM users WHERE email = $1",
            &[&email],
        )
        .await?;

    match row_opt {
        Some(row) => {
            let mut user = UserDb::default();
            user.id = row.get(0);
            user.email = row.get(1);
            user.password = row.get(2);

            return Ok(Some(user));
        }
        None => {
            return Ok(None);
        }
    }
}

pub async fn create_user(
    db_pool: &deadpool_postgres::Pool,
    user_db: UserDb,
) -> Result<i64, error::Error> {
    let client = db_pool.get().await?;

    // Insert the user into the database
    let row = client
        .query_one(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id",
            &[&user_db.email, &user_db.password],
        )
        .await?;

    // Extract and return the ID of the newly inserted user
    let id: i64 = row.get(0);
    Ok(id)
}
