pub mod common;
pub mod handlers;
pub mod repositories;
pub mod services;

use axum::{middleware, routing::get, routing::post, Router};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let shared_state = handlers::state::RootState::new();

    let public_router = Router::new()
        .route("/public/register", post(handlers::auth::register))
        .route("/public/login", post(handlers::auth::login));

    let auth_router = Router::new()
        .route("/users/self", get(handlers::user::get_user_self))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(handlers::auth::validate)));

    let app = Router::new()
        .merge(public_router)
        .merge(auth_router)
        .with_state(shared_state.clone());

    let port = 6000;
    println!("Listening on port {}", port);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
