use crate::validators;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
