use models::*;
use sea_orm::*;

use crate::validators::ValidationError;
pub struct Queries;

const INVALID_CREDETIALS: &str = "Invalid credentials";

impl Queries {
    pub async fn fetch_user_by_email(
        db: &DbConn,
        identity: &str,
    ) -> Result<users::Model, ValidationError> {
        users::Entity::find_by_email(identity.to_string())
            .one(db)
            .await?
            .ok_or_else(|| ValidationError::BadRequest(INVALID_CREDETIALS.to_string()))
    }

    pub async fn fetch_auth_methods_by_identifier(
        db: &DbConn,
        identity: &str,
    ) -> Result<users::Model, ValidationError> {
        let auth_method = auth_methods::Entity::find()
            .filter(auth_methods::Column::Identifier.eq(identity.to_string()))
            .one(db)
            .await?
            .ok_or_else(|| ValidationError::BadRequest(INVALID_CREDETIALS.to_string()))?;

        users::Entity::find_by_id(auth_method.user_id)
            .one(db)
            .await?
            .ok_or_else(|| ValidationError::BadRequest(INVALID_CREDETIALS.to_string()))
    }
}
