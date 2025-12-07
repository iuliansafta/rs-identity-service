use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::Utc;
use models::*;
use sea_orm::*;
use uuid::Uuid;

use crate::dto::CreateUser;

pub struct Mutations;

impl Mutations {
    pub async fn create_user(db: &DbConn, payload: CreateUser) -> Result<users::Model, DbErr> {
        let generated_id = Uuid::now_v7();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .map_err(|e| DbErr::Custom(format!("Password hashing failed: {}", e)))?
            .to_string();

        let now = Utc::now().naive_utc();

        let user = users::ActiveModel {
            id: Set(generated_id),
            email: Set(payload.email),
            password_hash: Set(Some(password_hash)),
            login_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
        };

        user.insert(db).await
    }
}
