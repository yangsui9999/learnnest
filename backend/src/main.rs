use salvo::prelude::*;
use std::error::Error;

use crate::config::AppConfig;
use crate::db::create_pool;
use crate::handler::{account::*, health::*, register::*};
use crate::middleware::auth::create_jwt_auth;

mod config;
mod db;
mod error;
mod handler;
mod middleware;
mod model;
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

    // 创建中间件
    let auth_middleware = create_jwt_auth(&config.jwt_secret);

    // 注入状态到路由
    let router: Router = Router::new()
        .hoop(affix_state::inject(pgpool))
        .hoop(affix_state::inject(config.clone()))
        .push(Router::with_path("health").get(health_check))
        .push(Router::with_path("api/account/register").post(register))
        .push(Router::with_path("api/account/login").post(login))
        //.push(Router::with_path("api/task").hoop(auth_middleware).get(list_tasks))
        ;

    // 启动服务器
    let addr: String = format!("{}:{}", config.server_host, config.server_port);
    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
