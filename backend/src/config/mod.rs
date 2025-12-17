use crate::error::AppError;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| AppError::EnvVar("DATABASE_URL".into()))?;
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "9000".to_string())
            .parse::<u16>()?;

        Ok(AppConfig {
            server_host,
            server_port,
            database_url,
        })
    }
}
