use crate::response::ApiResponse;
use salvo::http::StatusCode;
use salvo::prelude::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("请求参数错误: {0}")]
    BadRequest(String),

    #[error("环境变量 {0} 未设置")]
    EnvVar(String),

    #[error("端口号解析失败: {0}")]
    PortParse(#[from] std::num::ParseIntError),

    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("密码加密错误")]
    PasswordHash,

    #[error("密码解密错误")]
    PasswordVerify(String),

    #[error("密码错误")]
    PasswordError,

    #[error("登录token错误")]
    TokenError,
}

#[async_trait]
impl Writer for AppError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status = match &self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        res.status_code(status);
        res.render(Json(ApiResponse::<()>::error(
            status.as_u16() as i32,
            self.to_string(),
        )));
    }
}
