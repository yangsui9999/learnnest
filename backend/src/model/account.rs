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
    pub username: Option<String>,
    pub nickname: String,
}

// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// 登录返回
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub account: AccountResponse,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject: account_id
    pub exp: usize,  // 过期时间（时间戳）
}

// 数据库对应的实体
#[derive(Debug, sqlx::FromRow)]
#[expect(dead_code)]
pub struct Account {
    pub id: Uuid,
    pub username: Option<String>,
    pub nickname: String,
    pub password_hash: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub is_deleted: bool,
}
