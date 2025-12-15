use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validators;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrLoginUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(
        length(min = 10, max = 30),
        custom(function = "validators::utils::validate_password")
    )]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct IdentityAuthRequest {
    pub identifier: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthenticateUserRequest {
    #[validate(email)]
    pub identity: String,

    #[validate(length(min = 4, max = 30))]
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticatedUserResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub exp_time: u64,
    pub issued_at: i64,
}
