use crate::common::context::AppContext;
use crate::config::AppConfig;
use crate::db::create_pool;
use crate::handler::task::{create_task, delete_task, get_task, list_tasks, update_task};
use crate::handler::{account::*, health::*, register::*};
use crate::middleware::auth::create_jwt_auth;
use salvo::prelude::*;
use std::error::Error;
use std::sync::Arc;

mod common;
mod config;
mod db;
mod error;
mod handler;
mod middleware;
mod model;
mod repository;
mod response;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 加载环境变量
    dotenvy::dotenv()?;
    let config = AppConfig::from_env()?;

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 数据库 pool
    let pgpool = create_pool(&config).await?;
    let ctx = Arc::new(AppContext::new(pgpool));

    // 创建中间件
    let auth_middleware = create_jwt_auth(&config.jwt_secret);

    // 注入状态到路由
    let router: Router = Router::new()
        .hoop(affix_state::inject(ctx))
        .hoop(affix_state::inject(config.clone()))
        .push(Router::with_path("health").get(health_check))
        .push(Router::with_path("api/account/register").post(register))
        .push(Router::with_path("api/account/login").post(login))
        .push(
            Router::with_path("api")
                .hoop(auth_middleware)
                .push(Router::with_path("tasks").get(list_tasks).post(create_task))
                .push(
                    Router::with_path("tasks/{id}")
                        .get(get_task)
                        .put(update_task)
                        .delete(delete_task),
                ),
        );

    // 启动服务器
    let addr: String = format!("{}:{}", config.server_host, config.server_port);
    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
