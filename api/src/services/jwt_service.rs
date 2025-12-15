use crate::config::{Config, KeySource};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    identity: String,
    exp: u64,
    id: Uuid,
}

#[derive(Debug, Clone)]
pub struct JwtService {
    private_key: String,
    public_key: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    // pub token_id: Uuid,
}

impl JwtService {
    pub fn new(cfg: &Config) -> Self {
        match cfg.jwt_key_source {
            KeySource::EnvVar => JwtService {
                private_key: cfg.jwt_private_key.clone(),
                public_key: cfg.jwt_public_key.clone(),
            },
            KeySource::File => JwtService {
                private_key: cfg.jwt_private_key.clone(),
                public_key: cfg.jwt_public_key.clone(),
            },
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: &str,
        user_identity: String,
    ) -> anyhow::Result<String, jsonwebtoken::errors::Error> {
        let duration = Duration::from_mins(30);

        let claims = Claims {
            sub: user_id.to_string(),
            identity: user_identity,
            exp: duration.as_secs(),
            id: Uuid::now_v7(),
        };

        let file_contents =
            read_pem_file(self.private_key.as_str()).expect("Cannot find private key");
        let slices: &[u8] = &file_contents;

        let token = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(slices)?,
        )?;

        Ok(token)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: &str,
    ) -> anyhow::Result<String, jsonwebtoken::errors::Error> {
        let duration = Duration::from_hours(24);
        let claims = Claims {
            sub: user_id.to_string(),
            identity: "".to_string(),
            exp: duration.as_secs(),
            id: Uuid::now_v7(),
        };

        let file_contents =
            read_pem_file(self.private_key.as_str()).expect("Cannot find private key");
        let slices: &[u8] = &file_contents;

        let token = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(slices)?,
        )?;

        Ok(token)
    }

    pub fn generate_token_for_user(
        &self,
        user_id: String,
        user_identity: String,
    ) -> anyhow::Result<TokenResponse, jsonwebtoken::errors::Error> {
        let access_token = self.generate_access_token(user_id.as_str(), user_identity)?;
        let refresh_token = self.generate_refresh_token(user_id.as_str())?;

        let duration = Duration::from_mins(30);

        Ok(TokenResponse {
            access_token,
            refresh_token,
            expires_in: duration.as_secs(),
        })
    }

    pub fn validate_access_token(
        &self,
        token: &str,
    ) -> anyhow::Result<bool, jsonwebtoken::errors::Error> {
        let file_contents =
            read_pem_file(self.public_key.as_str()).expect("Cannot find private key");
        let slices: &[u8] = &file_contents;

        decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_pem(slices)?,
            &Validation::new(Algorithm::RS256),
        )?;

        // TODO: Implement claims validation logic

        Ok(true)
    }
}

pub fn read_pem_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file_path)
}
