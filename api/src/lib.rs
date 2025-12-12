use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod config;
pub mod dto;
pub mod handlers;
pub mod services;
pub mod validators;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub cfg: Arc<config::Config>,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("identity_service_api=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    // Load config
    let config = Arc::new(config::Config::from_env()?);

    // Connect to database
    let db = Database::connect(&config.database_url).await?;
    tracing::info!("Connected to database");

    // Create the state
    let state = Arc::new(AppState {
        db,
        cfg: config.clone(),
    });

    // Build routes
    let app = Router::new()
        .route("/health", get(|| async { "Ok" }))
        .route("/users", post(handlers::register))
        .route("/auth/login", post(handlers::login))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Run server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
