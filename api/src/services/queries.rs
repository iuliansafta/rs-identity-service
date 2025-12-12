use models::*;
use sea_orm::*;

use crate::validators::ValidationError;
pub struct Queries;

impl Queries {
    pub async fn fetch_user_by_email(
        db: &DbConn,
        identity: &str,
    ) -> Result<users::Model, ValidationError> {
        users::Entity::find_by_email(identity.to_string())
            .one(db)
            .await?
            .ok_or_else(|| ValidationError::BadRequest("Invalid credentials".to_string()))
    }
}
