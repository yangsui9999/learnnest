use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub account_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[sqlx(rename = "type")]
    pub task_type: String,
    pub subject: String,
    pub status: String,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub is_deleted: bool,
}

// 创建请求
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub task_type: String,
    pub subject: String,
    pub due_date: Option<DateTime<Utc>>,
}

// 响应
#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub task_type: String,
    pub subject: String,
    pub status: String,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub task_type: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
}