use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use std::sync::Arc;

use crate::services::JwtService;
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
        .map_err(|e| ValidationError::Internal(anyhow::anyhow!("Failed to create user: {}", e)))?;

    let response = dto::UserResponse {
        id: user.id.to_string(),
        email: user.email,
    };

    Ok(Json(json!(response)))
}

pub async fn init_login(
    State(_): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ValidationError> {
    Ok(Json(json!("{\"cici\":\"done\"}")))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<dto::AuthenticateUserRequest>,
) -> Result<Json<serde_json::Value>, ValidationError> {
    const INVALID_CREDENTIALS: &str = "Invalid credentials";

    let user = services::Queries::fetch_user_by_email(&state.db, &payload.identity).await?;

    let has_password = user
        .password_hash
        .ok_or_else(|| ValidationError::BadRequest(INVALID_CREDENTIALS.to_string()))?;

    let parsed_hash = PasswordHash::new(&has_password)
        .map_err(|e| ValidationError::PasswordHashError(e.to_string()))?;

    if Argon2::default()
        .verify_password(payload.code.as_bytes(), &parsed_hash)
        .is_ok()
    {
        return Err(ValidationError::BadRequest(INVALID_CREDENTIALS.to_string()));
    }

    let jwt = JwtService::new(state.cfg.jwt_secret.as_str());
    let token = jwt
        .generate_access_token(user.id.to_string(), user.email)
        .map_err(|e| ValidationError::JwtError(e.to_string()))?;

    let response = dto::AuthenticatedUserResponse {
        token,
        refresh_token: "".to_string(),
        exp_time: 1233434,
    };

    Ok(Json(json!(response)))
}
