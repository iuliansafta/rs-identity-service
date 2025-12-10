use std::env;

use anyhow::Ok;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: get_env_or_default("DATABASE_URL", None)?,
            host: get_env_or_default("HOST", Some("127.0.0.1"))?,
            port: get_env_or_default("PORT", Some("3000"))?.parse()?,
            jwt_secret: get_env_or_default("JWT_SECRET", None)?,
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
