use std::sync::Arc;

use crate::validators::{ValidatedJson, ValidationError};
use axum::extract::{Json, State};
use serde_json::json;

use crate::{AppState, dto, services};

pub async fn register(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<dto::CreateOrLoginUserRequest>,
) -> Result<Json<serde_json::Value>, ValidationError> {
    let user = services::Mutations::create_user(&state.db, payload)
        .await
        .expect("could not create the user");

    let response = dto::UserResponse {
        id: user.id.to_string(),
        email: user.email,
    };

    Ok(Json(json!(response)))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<dto::CreateOrLoginUserRequest>,
) -> Result<Json<serde_json::Value>, ValidationError> {
    todo!()
}
