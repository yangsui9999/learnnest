use salvo::{handler, prelude::JwtAuthDepotExt, writing::Json, Depot, Request};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::context::AppContext;
use crate::model::task::{CreateTaskRequest, UpdateTaskRequest};
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

    // 3. 写入数据
    let ctx = depot.obtain::<Arc<AppContext>>().unwrap();
    let task = ctx.services.task.create(account_id, &input).await?;

    Ok(Json(ApiResponse::success(task.into())))
}

#[handler]
pub async fn list_tasks(depot: &mut Depot) -> ApiResult<Vec<TaskResponse>> {
    let account_id = get_account_id(depot);
    let ctx = depot.obtain::<Arc<AppContext>>().unwrap();
    let tasks = ctx.services.task.list(account_id).await?;
    let responses = tasks.into_iter().map(Into::into).collect();
    Ok(Json(ApiResponse::success(responses)))
}

#[handler]
pub async fn get_task(req: &mut Request, depot: &mut Depot) -> ApiResult<TaskResponse> {
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;

    let account_id = get_account_id(depot);
    let ctx = depot.obtain::<Arc<AppContext>>().unwrap();
    let task = ctx.services.task.get(task_id, account_id).await?;

    Ok(Json(ApiResponse::success(task.into())))
}

#[handler]
pub async fn update_task(req: &mut Request, depot: &mut Depot) -> ApiResult<()> {
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let account_id = get_account_id(depot);

    let input: UpdateTaskRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let ctx = depot.obtain::<Arc<AppContext>>().unwrap();
    ctx.services.task.update(task_id, account_id, input).await?;

    Ok(Json(ApiResponse::ok()))
}

#[handler]
pub async fn delete_task(req: &mut Request, depot: &mut Depot) -> ApiResult<()> {
    let task_id = req.param::<String>("id").unwrap();
    let task_id = Uuid::parse_str(&task_id).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let account_id = get_account_id(depot);

    let ctx = depot.obtain::<Arc<AppContext>>().unwrap();
    ctx.services.task.delete(task_id, account_id).await?;

    Ok(Json(ApiResponse::ok()))
}
