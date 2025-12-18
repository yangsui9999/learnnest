use salvo::prelude::*;
use sqlx::PgPool;
use serde::Serialize;

use crate::response::ApiResponse;

#[derive(Serialize)]
pub struct HealthData {
    pub app: String,
    pub db: String,
}

#[handler]
pub async fn health_check(depot: &mut Depot, res: &mut Response) {
    // 从 depot 获取连接池
    let pool = depot.obtain::<PgPool>().unwrap();

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

    res.render(Json(ApiResponse::success(data)));
}