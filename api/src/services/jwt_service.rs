use crate::config::KeySource;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    sub: String,
    identity: String,
    exp: u64,
    id: Uuid,
}

#[derive(Debug, Clone)]
pub struct JwtService {
    private_key: String,
    public_key: String,
    key_source: KeySource,
    encoding_algo: Algorithm,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub refresh_token_id: Option<Uuid>,
}

impl JwtService {
    pub fn new(private_key_path: &str, public_key_path: &str, key_source: KeySource) -> Self {
        let encoding_algo = match key_source {
            KeySource::Hmac => Algorithm::HS256,
            KeySource::Rsa => Algorithm::RS256,
        };

        JwtService {
            private_key: private_key_path.to_string(),
            public_key: public_key_path.to_string(),
            key_source,
            encoding_algo,
        }
    }

    fn get_token_by_source(
        &self,
        claims: &Claims,
    ) -> anyhow::Result<String, jsonwebtoken::errors::Error> {
        let token: String = match self.key_source {
            KeySource::Hmac => {
                let key = EncodingKey::from_secret(self.private_key.as_bytes());
                encode(&Header::new(self.encoding_algo), &claims, &key)?
            }
            KeySource::Rsa => {
                let file_contents =
                    read_pem_file(self.private_key.as_str()).expect("Cannot find private key");
                let slices: &[u8] = &file_contents;
                encode(
                    &Header::new(self.encoding_algo),
                    &claims,
                    &EncodingKey::from_rsa_pem(slices)?,
                )?
            }
        };

        Ok(token)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: &str,
    ) -> anyhow::Result<(String, Uuid), jsonwebtoken::errors::Error> {
        let duration = Duration::from_hours(24);
        let expiration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + duration.as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            identity: "".to_string(),
            exp: expiration,
            id: Uuid::now_v7(),
        };

        let token = self.get_token_by_source(&claims)?;

        Ok((token, claims.id))
    }

    pub fn generate_access_token(
        &self,
        user_id: &str,
        user_identity: &str,
    ) -> anyhow::Result<(String, Uuid), jsonwebtoken::errors::Error> {
        let duration = Duration::from_hours(24);
        let expiration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + duration.as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            identity: user_identity.to_string(),
            exp: expiration,
            id: Uuid::now_v7(),
        };

        let token = self.get_token_by_source(&claims)?;

        Ok((token, claims.id))
    }

    pub fn generate_token_for_user(
        &self,
        user_id: String,
        user_identity: String,
    ) -> anyhow::Result<TokenResponse, jsonwebtoken::errors::Error> {
        let access_token = self.generate_access_token(user_id.as_str(), user_identity.as_str())?;
        let refresh_token = self.generate_refresh_token(user_id.as_str())?;

        let duration = Duration::from_mins(30);

        Ok(TokenResponse {
            access_token: access_token.0,
            refresh_token: refresh_token.0,
            refresh_token_id: Some(refresh_token.1),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_access_token() {
        let jwt_service = JwtService::new(
            "test_private_key_str",
            "test_public_key_str",
            KeySource::Hmac,
        );

        let user_id = "user123";
        let user_identity = "user@example.com";

        let token_response = jwt_service
            .generate_token_for_user(user_id.to_string(), user_identity.to_string())
            .expect("Should generate token for valid user");

        assert!(!token_response.access_token.is_empty());
        assert!(!token_response.refresh_token.is_empty());
        assert!(token_response.refresh_token_id.is_some());
        assert_eq!(token_response.expires_in, 1800);
    }

    #[test]
    fn test_validate_access_token_hmac() {
        let jwt_service = JwtService::new("test_secret_key", "test_secret_key", KeySource::Hmac);
        let user_id = "user123";
        let user_identity = "user@example.com";

        let (access_token, _) = jwt_service
            .generate_access_token(user_id, user_identity)
            .expect("Should decode token for valid user");

        // Validate the token
        let validation_key = DecodingKey::from_secret("test_secret_key".as_bytes());
        let result = decode::<Claims>(
            &access_token,
            &validation_key,
            &Validation::new(Algorithm::HS256),
        );

        assert!(result.is_ok());
    }
}
