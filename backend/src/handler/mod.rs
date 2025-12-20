use crate::app::context::AppContext;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::model::account::Claims;
use salvo::{prelude::JwtAuthDepotExt, Depot, Request};
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub mod account;
pub mod health;
pub mod register;
pub mod task;

/// Handler 层内部工具：安全获取 depot 中的依赖
pub(crate) fn require_state<T: Send + Sync + 'static>(depot: &Depot) -> Result<&T, AppError> {
    depot.obtain::<T>().map_err(|e| {
        let type_name = std::any::type_name::<T>();
        tracing::error!(err = ?e, "依赖未注入: {}", type_name);
        debug_assert!(false, "missing depot state: {}", type_name);
        AppError::Internal
    })
}

pub(crate) trait DepotExt {
    fn pool(&self) -> Result<&PgPool, AppError>;
    fn app_context(&self) -> Result<&Arc<AppContext>, AppError>;
    fn current_account_id(&self) -> Result<Uuid, AppError>;
    fn app_config(&self) -> Result<&AppConfig, AppError>;
}

impl DepotExt for Depot {
    fn pool(&self) -> Result<&PgPool, AppError> {
        require_state::<PgPool>(self)
    }

    fn app_context(&self) -> Result<&Arc<AppContext>, AppError> {
        require_state::<Arc<AppContext>>(self)
    }

    // 从 JWT 中获取 ID
    fn current_account_id(&self) -> Result<Uuid, AppError> {
        let claims = &self
            .jwt_auth_data::<Claims>()
            .ok_or(AppError::Unauthorized)?;

        let account_id = Uuid::parse_str(&claims.claims.sub).map_err(|_| AppError::Unauthorized)?;

        Ok(account_id)
    }

    fn app_config(&self) -> Result<&AppConfig, AppError> {
        require_state::<AppConfig>(self)
    }
}

pub(crate) trait RequestExt {
    fn require_uuid_param(&self, name: &str) -> Result<Uuid, AppError>;
    async fn parse_request_body<T: DeserializeOwned>(&mut self) -> Result<T, AppError>;
}

impl RequestExt for Request {
    fn require_uuid_param(&self, name: &str) -> Result<Uuid, AppError> {
        let value = &self
            .param::<String>(name)
            .ok_or_else(|| AppError::BadRequest(format!("{} 缺少参数", name)))?;
        Uuid::parse_str(&value).map_err(|_| AppError::BadRequest(format!("无效的 UUID: {}", name)))
    }

    async fn parse_request_body<T: DeserializeOwned>(&mut self) -> Result<T, AppError> {
        self.parse_json::<T>()
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))
    }
}
