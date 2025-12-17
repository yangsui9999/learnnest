use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::AppConfig;


pub async fn create_pool(config: &AppConfig) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_conn)
        .acquire_timeout(Duration::from_secs(config.db_connect_timeout_secs))
        .idle_timeout(Duration::from_secs(config.db_idle_timeout))
        .connect(&config.db_url)
        .await?;

    Ok(pool)
}