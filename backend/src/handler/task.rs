use salvo::{handler, writing::Json, Depot, Request};

use crate::handler::{DepotExt, RequestExt};
use crate::model::task::{CreateTaskRequest, UpdateTaskRequest};
use crate::{
    model::task::TaskResponse,
    response::{ApiResponse, ApiResult},
};

#[handler]
pub async fn create_task(req: &mut Request, depot: &mut Depot) -> ApiResult<TaskResponse> {
    // 解析请求体
    let task_req = req.parse_request_body::<CreateTaskRequest>().await?;

    let ctx = depot.app_context()?;
    let account_id = depot.current_account_id()?;
    let task = ctx.services.task.create(account_id, &task_req).await?;

    Ok(Json(ApiResponse::success(task.into())))
}

#[handler]
pub async fn list_tasks(depot: &mut Depot) -> ApiResult<Vec<TaskResponse>> {
    let ctx = depot.app_context()?;
    let account_id = depot.current_account_id()?;
    let tasks = ctx.services.task.list(account_id).await?;
    let responses = tasks.into_iter().map(Into::into).collect();
    Ok(Json(ApiResponse::success(responses)))
}

#[handler]
pub async fn get_task(req: &mut Request, depot: &mut Depot) -> ApiResult<TaskResponse> {
    let ctx = depot.app_context()?;
    let account_id = depot.current_account_id()?;
    let task_id = req.require_uuid_param("id")?;

    let task = ctx.services.task.get(task_id, account_id).await?;

    Ok(Json(ApiResponse::success(task.into())))
}

#[handler]
pub async fn update_task(req: &mut Request, depot: &mut Depot) -> ApiResult<()> {
    let update_req = req.parse_request_body::<UpdateTaskRequest>().await?;

    let ctx = depot.app_context()?;
    let account_id = depot.current_account_id()?;
    let task_id = req.require_uuid_param("id")?;
    ctx.services
        .task
        .update(task_id, account_id, update_req)
        .await?;

    Ok(Json(ApiResponse::ok()))
}

#[handler]
pub async fn delete_task(req: &mut Request, depot: &mut Depot) -> ApiResult<()> {
    let ctx = depot.app_context()?;
    let account_id = depot.current_account_id()?;
    let task_id = req.require_uuid_param("id")?;

    ctx.services.task.delete(task_id, account_id).await?;

    Ok(Json(ApiResponse::ok()))
}
