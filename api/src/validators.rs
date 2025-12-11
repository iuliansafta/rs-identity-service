use axum::{
    Json,
    extract::{FromRequest, Json as JsonExtractor, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("Internal Error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            ValidationError::Internal(_) => {
                let payload = ErrorMessage::new("internal_error", "Something went wrong");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
            }
            ValidationError::ValidationError(errs) => (
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse::from_errors(errs)),
            )
                .into_response(),
            ValidationError::BadRequest(_) => {
                let payload = ErrorMessage::new("internal_error", "Something went wrong");
                tracing::error!("validation(400): {}", self.to_string());
                (StatusCode::BAD_REQUEST, Json(payload)).into_response()
            }
            _ => {
                let payload = ErrorMessage::new("internal_error", "Something went wrong");
                tracing::error!("validation(500): {}", self.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
            }
        }
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    code: String,
    message: String,
}

impl ErrorMessage {
    fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
        }
    }
}

#[derive(Serialize, Debug)]
struct ValidationErrorResponse {
    code: String,
    message: String,
    errors: Vec<FieldError>,
}

#[derive(Serialize, Debug)]
struct FieldError {
    field: String,
    messages: Vec<String>,
}

impl ValidationErrorResponse {
    fn from_errors(errors: validator::ValidationErrors) -> Self {
        let mut list = Vec::new();

        for (field, kind) in errors.field_errors().iter() {
            let msgs = kind
                .iter()
                .map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| e.code.to_string())
                })
                .collect::<Vec<_>>();
            list.push(FieldError {
                field: field.to_string(),
                messages: msgs,
            });
        }

        ValidationErrorResponse {
            code: "validation_error".to_string(),
            message: "Request validation failed".to_string(),
            errors: list,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    JsonExtractor<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ValidationError;

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let JsonExtractor(value) = JsonExtractor::<T>::from_request(req, state).await?;
        Ok(ValidatedJson(validate_and_wrap(value)?))
    }
}

fn validate_and_wrap<T: Validate>(value: T) -> Result<T, ValidationError> {
    value.validate()?;
    Ok(value)
}

// Helper modules
pub mod utils {
    use validator::ValidationError;
    pub fn validate_password(pw: &str) -> Result<(), ValidationError> {
        if pw.len() < 10 {
            return Err(ValidationError::new("password_too_short"));
        }

        let has_alpha = pw.chars().any(|c| c.is_ascii_alphabetic());
        let has_digits = pw.chars().any(|c| c.is_ascii_digit());

        if !(has_alpha && has_digits) {
            return Err(ValidationError::new("password_week"));
        }

        Ok(())
    }
}
