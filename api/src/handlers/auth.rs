use std::sync::Arc;

use axum::extract::{Json, State};
use serde_json::json;

use crate::{
    AppState,
    dto::{self, CreateUser},
    error::AppError,
    services,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = services::Mutations::create_user(&state.db, payload)
        .await
        .expect("could not create the user");

    let response = dto::UserResponse {
        id: user.id.to_string(),
        email: user.email,
    };

    Ok(Json(json!(response)))
}
