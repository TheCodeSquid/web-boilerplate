use std::net::IpAddr;

use dotenvy::dotenv;
use http::header::HeaderValue;
use humantime::Duration;
use service::config::*;

pub struct ApiConfig {
    pub log: String,

    pub db_url: String,

    pub addr: IpAddr,
    pub port: u16,

    pub cors_origins: CommaVec<HeaderValue>,

    pub session_secret: String,
    pub session_lifetime: Duration,

    pub pepper: String,
    pub pepper_old: Option<String>,
}

impl ApiConfig {
    pub fn from_env() -> Result<ApiConfig, ConfigError> {
        dotenv().ok();

        Ok(ApiConfig {
            log: var_or("LOG", "info".to_string())?,

            db_url: var("DATABASE_URL")?,

            addr: var_or("ADDR", [127, 0, 0, 1].into())?,
            port: var_or("PORT", 8000)?,

            cors_origins: var("CORS_ORIGINS")?,

            session_secret: var("SESSION_SECRET")?,
            session_lifetime: var_or(
                "SESSION_LIFETIME",
                std::time::Duration::from_secs(30).into(),
            )?,

            pepper: var("PEPPER")?,
            pepper_old: var("PEPPER_OLD").ok(),
        })
    }
}
