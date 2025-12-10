use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("validation error")]
    Validation(#[from] ValidationErrors),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("Internal Error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Internal(_) => {
                let payload = ErrorMessage::new("internal_error", "Something went wrong");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
            }
            AppError::BadRequest(_) => {
                let payload = ErrorMessage::new("internal_error", "Something went wrong");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
            }
            AppError::Validation(errs) => {
                println!("erros :{:?}", errs);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ValidationErrorResponse::from_errors(errs)),
                )
                    .into_response()
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
    fn from_errors(errors: ValidationErrors) -> Self {
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
