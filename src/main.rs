pub mod common;
pub mod config;
pub mod handlers;
pub mod repositories;
pub mod services;

use axum::{middleware, routing::get, routing::post, Router};
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod};
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tracing_subscriber;
use tracing_subscriber::FmtSubscriber;

// Function to configure logging based on the configuration
fn setup_logging(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => "trace",
        "debug" => "debug",
        "info" => "info",
        "warn" => "warn",
        "error" => "error",
        _ => "info", // Default level
    };

    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder().with_env_filter(level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}

fn create_db_pool() -> deadpool_postgres::Pool {
    let c = config::config::get_config();

    let mut cfg = Config::new();
    cfg.dbname = Some(c.database.db_name.to_string());
    cfg.user = Some(c.database.user.to_string());
    cfg.password = Some(c.database.password.to_string());
    cfg.host = Some(c.database.host.to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    // Set the pool size here
    cfg.pool = Some(deadpool_postgres::PoolConfig {
        max_size: 10, // Start with 10 connections
        ..Default::default()
    });

    cfg.create_pool(None, NoTls).unwrap()
}

#[tokio::main]
async fn main() {
    let c = config::config::get_config();
    setup_logging(&c.log_level);
    let db_pool = create_db_pool();
    let shared_state = handlers::state::RootState::new(db_pool);

    let public_router = Router::new()
        .route("/public/register", post(handlers::auth::register))
        .route("/public/login", post(handlers::auth::login));

    let auth_router = Router::new()
        .route("/auth/users", get(handlers::user::get_user_self))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(handlers::auth::validate)));

    let app = Router::new()
        .merge(public_router)
        .merge(auth_router)
        .with_state(shared_state)
        .layer(
            ServiceBuilder::new().layer(middleware::from_fn(handlers::utils::common_middleware)),
        );

    tracing::info!("Listening on port {}", c.port);
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", c.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
