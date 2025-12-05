use std::sync::Arc;

use axum::extract::{Json, State};

use crate::{AppState, error::AppError};

pub async fn register(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!();
}
