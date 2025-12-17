use crate::error::AppError;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub db_url: String,
    pub db_max_conn: u32,
    pub db_connect_timeout_secs: u64,
    pub db_idle_timeout: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let db_url =
            env::var("DATABASE_URL").map_err(|_| AppError::EnvVar("DATABASE_URL".into()))?;
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "9000".to_string())
            .parse::<u16>()?;
        let db_max_conn = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()?;
        let db_connect_timeout_secs: u64 = env::var("DATABASE_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()?;
        let db_idle_timeout: u64 = env::var("DATABASE_IDLE_TIMEOUT")
            .unwrap_or_else(|_| "600".to_string())
            .parse::<u64>()?;

        Ok(AppConfig {
            server_host,
            server_port,
            db_url,
            db_max_conn,
            db_connect_timeout_secs,
            db_idle_timeout
        })
    }
}
