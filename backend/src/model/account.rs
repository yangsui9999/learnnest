use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 注册请求
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
}

// 注册响应
#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub username: String,
    pub nickname: String,
}

// 数据库对应的实体
#[derive(Debug, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub username: Option<String>,
    pub nickname: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub is_deleted: bool,
}
