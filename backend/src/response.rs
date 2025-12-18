use crate::error::AppError;
use salvo::writing::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

// 带数据的方法
impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        ApiResponse {
            code,
            message: msg.into(),
            data: None,
        }
    }
}

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, AppError>;

// 无数据的方法
impl ApiResponse<()> {
    pub fn ok() -> Self {
        ApiResponse {
            code: 0,
            message: "ok".to_string(),
            data: None,
        }
    }
}
