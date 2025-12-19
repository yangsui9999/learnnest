use crate::response::ApiResponse;
use salvo::http::StatusCode;
use salvo::prelude::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("请求参数错误: {0}")]
    BadRequest(String),

    #[error("资源不存在")]
    NotFound,

    #[error("用户名或密码错误")]
    PasswordError,

    #[error("未授权访问")]
    Unauthorized,

    // === 内部错误（统一提示）===
    #[error("系统繁忙，请稍后重试")]
    Internal,

    // === 启动时错误（不走 HTTP）===
    #[error("环境变量 {0} 未设置")]
    EnvVar(String),

    #[error("配置解析失败: {0}")]
    ConfigParse(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => {
                tracing::error!("数据库错误: {:?}", err);
                AppError::Internal
            }
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        tracing::error!("解析错误: {:?}", err);
        AppError::Internal
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status = match &self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::PasswordError => StatusCode::UNAUTHORIZED,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        res.status_code(status);
        res.render(Json(ApiResponse::<()>::error(
            status.as_u16() as i32,
            self.to_string(),
        )));
    }
}
