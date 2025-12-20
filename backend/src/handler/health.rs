use crate::handler::DepotExt;
use crate::response::{ApiResponse, ApiResult};
use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthData {
    pub app: String,
    pub db: String,
}

#[handler]
pub async fn health_check(depot: &mut Depot) -> ApiResult<HealthData> {
    // 从 depot 获取连接池
    let pool = depot.pool()?;

    // 测试数据库连接
    let db_status = sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_| "ok")
        .unwrap_or("fail");

    // 返回 jsn
    let data = HealthData {
        app: "ok".to_string(),
        db: db_status.into(),
    };

    Ok(Json(ApiResponse::success(data)))
}
