use std::sync::Arc;

use axum::{Router, routing::get};
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load config
    let config = config::Config::from_env()?;

    // Connect to database
    let db = Database::connect(&config.database_url).await?;
    tracing::info!("Connected to database");

    // Create the state
    let state = Arc::new(AppState { db });

    // Build routes
    let app = Router::new()
        .route("/health", get(|| async { "Ok" }))
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
