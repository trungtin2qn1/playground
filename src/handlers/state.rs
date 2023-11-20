#[derive(Clone)]
pub struct RootState {
    pub db: sled::Db,
}

impl RootState {
    pub fn new() -> Self {
        let db: sled::Db = sled::open("database").unwrap();
        RootState { db }
    }
}
