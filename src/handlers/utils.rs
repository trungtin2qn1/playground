use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Response {
    message: String,
}

impl Response {
    pub fn new(message: String) -> Self {
        Response { message }
    }
}
