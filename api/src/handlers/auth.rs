use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use chrono::{DateTime, Utc};
use std::{sync::Arc, time::SystemTime};

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
    // let auth_method = services::Queries
    todo!()
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
        .is_err()
    {
        return Err(ValidationError::BadRequest(INVALID_CREDENTIALS.to_string()));
    }

    let token = state
        .jwt_service
        .generate_token_for_user(user.id.to_string(), user.email)
        .map_err(|e| ValidationError::JwtError(e.to_string()))?;

    let now = SystemTime::now();
    let dt_utc: DateTime<Utc> = now.into();

    let response = dto::AuthenticatedUserResponse {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
        exp_time: token.expires_in,
        issued_at: dt_utc.timestamp(),
    };

    Ok(Json(json!(response)))
}
