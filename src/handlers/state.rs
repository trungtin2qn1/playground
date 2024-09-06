#[derive(Clone)]
pub struct RootState {
    pub db_pool: deadpool_postgres::Pool,
}

impl RootState {
    pub fn new(db_pool: deadpool_postgres::Pool) -> Self {
        RootState { db_pool }
    }
}
