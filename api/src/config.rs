use std::{env, fmt};

use anyhow::Ok;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl Environment {
    pub fn from_env() -> Self {
        let val = std::env::var("AXUM_ENV")
            .ok()
            .or_else(|| std::env::var("RUST_ENV").ok())
            .unwrap_or_else(|| "dev".to_string())
            .to_lowercase();

        match val.as_str() {
            "prod" | "production" => Environment::Prod,
            "staging" => Environment::Staging,
            _ => Environment::Dev,
        }
    }

    pub fn is_prod(&self) -> bool {
        matches!(self, Environment::Prod)
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Dev => write!(f, "dev"),
            Environment::Staging => write!(f, "staging"),
            Environment::Prod => write!(f, "prod"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeySource {
    Rsa,
    Hmac,
}

impl KeySource {
    pub fn from_env() -> Self {
        match std::env::var("JWT_KEY_SOURCE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "rsa" => KeySource::Rsa,
            "hmac" => KeySource::Hmac,
            _ => KeySource::Hmac,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_env: Environment,
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_private_key: String,
    pub jwt_public_key: String,
    pub jwt_key_source: KeySource,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let env = Environment::from_env();

        if !env.is_prod() {
            let _ = dotenvy::dotenv();
        }

        Ok(Self {
            app_env: Environment::from_env(),
            database_url: get_env_or_default("DATABASE_URL", None)?,
            host: get_env_or_default("HOST", Some("127.0.0.1"))?,
            port: get_env_or_default("PORT", Some("3000"))?.parse()?,
            jwt_private_key: get_env_or_default("JWT_PRIVATE_KEY_PATH", None)?,
            jwt_public_key: get_env_or_default("JWT_PUBLIC_KEY_PATH", None)?,
            jwt_key_source: KeySource::from_env(),
        })
    }
}

fn get_env_or_default(key: &str, default: Option<&str>) -> anyhow::Result<String> {
    env::var(key.to_uppercase())
        .ok()
        .filter(|s| !s.is_empty())
        .or(default.map(String::from))
        .ok_or_else(|| anyhow::anyhow!("Missing required env var: {}", key))
}
