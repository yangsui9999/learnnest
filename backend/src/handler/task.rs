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

    Ok(Json(ApiResponse::success(task.into())))
}

#[handler]
pub async fn list_tasks(depot: &mut Depot) -> ApiResult<Vec<TaskResponse>> {
    // 查询当前用户，未删除的任务
    let account_id = get_account_id(depot);

    let pool = depot.obtain::<PgPool>().unwrap();

    let tasks = TaskService::new(pool.clone()).list(account_id).await?;

    let responses = tasks.into_iter().map(Into::into).collect();

    Ok(Json(ApiResponse::success(responses)))
}

#[handler]
pub async fn get_task(req: &mut Request, depot: &mut Depot) -> ApiResult<TaskResponse> {
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let pool = depot.obtain::<PgPool>().unwrap();

    let task = TaskService::new(pool.clone())
        .get(task_id, account_id)
        .await?;

    Ok(Json(ApiResponse::success(task.into())))
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
        .update(task_id, account_id, input)
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
        .delete(task_id, account_id)
        .await?;

    Ok(Json(ApiResponse::ok()))
}
