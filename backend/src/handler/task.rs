use salvo::{handler, prelude::JwtAuthDepotExt, writing::Json, Depot, Request};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::task::{CreateTaskRequest, UpdateTaskRequest};
use crate::service::task::TaskService;
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

    // 3. 写入数据
    let task = TaskService::new(pool.clone())
        .create(account_id, &input)
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
pub async fn list_tasks(depot: &mut Depot) -> ApiResult<Vec<TaskResponse>> {
    // 查询当前用户，未删除的任务
    let account_id = get_account_id(depot);

    let pool = depot.obtain::<PgPool>().unwrap();

    let tasks = TaskService::new(pool.clone())
        .get_task_list_by_account_id(account_id)
        .await?;

    let responses = tasks
        .into_iter()
        .map(|t| TaskResponse {
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
pub async fn get_task(req: &mut Request, depot: &mut Depot) -> ApiResult<TaskResponse> {
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let pool = depot.obtain::<PgPool>().unwrap();

    let task = TaskService::new(pool.clone())
        .get_by_task_id_account_id(task_id, account_id)
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
pub async fn update_task(req: &mut Request, depot: &mut Depot) -> ApiResult<()> {
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let pool = depot.obtain::<PgPool>().unwrap();

    let input: UpdateTaskRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    TaskService::new(pool.clone())
        .update_by_task_id_account_id(task_id, account_id, input)
        .await?;

    Ok(Json(ApiResponse::ok()))
}

#[handler]
pub async fn delete_task(req: &mut Request, depot: &mut Depot) -> ApiResult<()> {
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let pool = depot.obtain::<PgPool>().unwrap();

    TaskService::new(pool.clone())
        .delete_by_task_id_account_id(task_id, account_id)
        .await?;

    Ok(Json(ApiResponse::ok()))
}
