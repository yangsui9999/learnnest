use salvo::prelude::*;
use std::error::Error;

use crate::config::AppConfig;
use crate::db::create_pool;
use crate::handler::health::*;

mod config;
mod db;
mod error;
mod handler;
mod response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 加载环境变量
    dotenvy::dotenv()?;
    let config = AppConfig::from_env()?;

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 数据库 pool
    let pgpool = create_pool(&config).await?;

    // 注入状态到路由
    let router: Router = Router::new()
        .hoop(affix_state::inject(pgpool))
        .push(Router::with_path("health").get(health_check));

    // 启动服务器
    let addr: String = format!("{}:{}", config.server_host, config.server_port);
    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
