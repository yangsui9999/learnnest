use salvo::{handler, prelude::JwtAuthDepotExt, writing::Json, Depot, Request};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::{
    error::AppError,
    model::{account::Claims, task::TaskResponse},
    response::{ApiResponse, ApiResult},
};

fn get_account_id(depot: &mut Depot) -> Uuid {
    // 从 JWT 获取 account_id
    let claims = depot.jwt_auth_data::<Claims>().unwrap();
    let account_id = Uuid::parse_str(&claims.claims.sub).unwrap();
    account_id
}

#[handler]
pub async fn create_task(req: &mut Request, depot: &mut Depot) -> ApiResult<TaskResponse> {
    // 从 JWT 获取 account_id
    let account_id = get_account_id(depot);

    // 解析请求体
    let input = req
        .parse_json::<CreateTaskRequest>()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let pool = depot.obtain::<PgPool>().unwrap();

    let uid = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();

    // 3. 写入数据
    let task = sqlx::query_as!(
        Task,
        r#"INSERT INTO task(id, account_id, title, description, type, subject, status, due_date, completed_at, source, created_at, created_by, updated_at, updated_by, is_deleted)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
           RETURNING id, account_id, title, description,
                     type AS task_type,
                     subject, status, due_date, completed_at, source,
                     created_at, created_by, updated_at, updated_by, is_deleted"#,
        uid,
        account_id,
        input.title,
        input.description,
        input.task_type,
        input.subject,
        "active",
        input.due_date,
        None::<chrono::DateTime<chrono::Utc>>,
        "manual",
        now,
        account_id,
        now,
        account_id,
        false,
    ).fetch_one(pool).await?;

    let response = TaskResponse {
        id: task.id,
        title: task.title,
        description: task.description,
        task_type: task.task_type,
        subject: task.subject,
        status: task.status,
        due_date: task.due_date,
        completed_at: task.completed_at,
        created_at: task.created_at,
    };

    Ok(Json(ApiResponse::success(response)))
}

#[handler]
pub async fn list_tasks(depot: &mut Depot) -> ApiResult<Vec<TaskResponse>> {
    // 查询当前用户，未删除的任务
    let account_id = get_account_id(depot);

    let pool = depot.obtain::<PgPool>().unwrap();

    let tasks = sqlx::query_as!(Task, 
        r#"SELECT id, account_id, title, description,
                    type AS task_type,
                    subject, status, due_date, completed_at, source,
                    created_at, created_by, updated_at, updated_by, is_deleted
             FROM task 
             WHERE account_id = $1 AND is_deleted = false
             ORDER BY created_at DESC
        "#,
        account_id
    )
    .fetch_all(pool)
    .await?;

    let responses = tasks.into_iter().map(|t| TaskResponse {
        id: t.id,
        title: t.title,
        description: t.description,
        task_type: t.task_type,
        subject: t.subject,
        status: t.status,
        due_date: t.due_date,
        completed_at: t.completed_at,
        created_at: t.created_at,
    })
    .collect();

    Ok(Json(ApiResponse::success(responses)))

}

#[handler]
pub async fn get_task(req: &mut Request, depot: &mut Depot)->ApiResult<TaskResponse>{
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let pool = depot.obtain::<PgPool>().unwrap();
    
    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT id, account_id, title, description,
            type AS task_type,
            subject, status, due_date, completed_at, source,
            created_at, created_by, updated_at, updated_by, is_deleted
        FROM task
        WHERE id = $1 AND account_id = $2 AND is_deleted = false
        "#,
        task_id,
        account_id
    )
    .fetch_one(pool)
    .await?;
    
    let response = TaskResponse {
        id: task.id,
        title: task.title,
        description: task.description,
        task_type: task.task_type,
        subject: task.subject,
        status: task.status,
        due_date: task.due_date,
        completed_at: task.completed_at,
        created_at: task.created_at,
    };

    Ok(Json(ApiResponse::success(response)))
}

#[handler]
pub async fn update_task(req: &mut Request, depot: &mut Depot)->ApiResult<()>{
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let pool = depot.obtain::<PgPool>().unwrap();
    let now = chrono::Utc::now();

    let input: UpdateTaskRequest = req.parse_json().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

    sqlx::query!(
        r#"
        UPDATE task SET 
              title = COALESCE($1, title),
              description = COALESCE($2, description),
              type = COALESCE($3, type),
              subject = COALESCE($4, subject),
              status = COALESCE($5, status),
              due_date = COALESCE($6, due_date),
              updated_at = $7,
              updated_by = $8
          WHERE id = $9 AND account_id = $10
        "#,
        input.title,
          input.description,
          input.task_type,
          input.subject,
          input.status,
          input.due_date,
          now,
          account_id,
          task_id,
          account_id
    ).execute(pool).await?;

    Ok(Json(ApiResponse::ok()))
}

#[handler]
pub async fn delete_task(req: &mut Request, depot: &mut Depot)->ApiResult<()>{
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let pool = depot.obtain::<PgPool>().unwrap();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        UPDATE task SET is_deleted = true, updated_at=$1, updated_by=$2 where id = $3 AND account_id = $4
        "#,
        now,
        account_id,
        task_id,
        account_id
    )
    .execute(pool)
    .await?;

    Ok(Json(ApiResponse::ok()))
}