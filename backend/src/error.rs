use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("环境变量 {0} 未设置")]
    EnvVar(String),

    #[error("端口号解析失败: {0}")]
    PortParse(#[from] std::num::ParseIntError),
}
