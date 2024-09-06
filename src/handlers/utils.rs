use axum::body::Body;
use axum::{http::HeaderValue, http::Request, middleware::Next};
use serde::Serialize;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Response {
    message: String,
}

impl Response {
    pub fn new(message: String) -> Self {
        Response { message }
    }
}

pub async fn common_middleware(req: Request<Body>, next: Next) -> axum::response::Response {
    let request_id = Uuid::new_v4();
    let method = req.method().as_str().to_string();
    let url = req.uri().to_string();

    let mut req = req;
    req.extensions_mut().insert(request_id.to_string());

    tracing::debug!(
        "Request request_id: {}, method: {}, url: {}",
        request_id,
        method,
        url
    );

    let span =
        tracing::info_span!("request", request_id = %request_id, method = %method, url = %url);

    let mut response = next.run(req).instrument(span).await;

    tracing::debug!(
        "Response request_id: {}, method: {}, url: {}, status: {}",
        request_id,
        method,
        url,
        response.status(),
    );

    response.headers_mut().insert(
        "request_id",
        HeaderValue::from_str(&request_id.to_string()).unwrap(),
    );

    response
}
