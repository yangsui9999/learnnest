use crate::app::from_pool::FromPool;
use crate::repository::account::AccountRepository;
use crate::repository::task::TaskRepository;
use crate::service::account::AccountService;
use crate::service::task::TaskService;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Repos {
    pub task: TaskRepository,
    pub account: AccountRepository,
    // 其它的 repo
}
impl Repos {
    pub fn new(pool: PgPool) -> Self {
        Self {
            task: TaskRepository::from_pool(pool.clone()),
            account: AccountRepository::from_pool(pool.clone()),
        }
    }
}

#[derive(Clone)]
pub struct Services {
    pub task: TaskService,
    pub account: AccountService,
    // 其它的 service
}

impl Services {
    pub fn new(repos: Repos) -> Self {
        Self {
            task: TaskService::new(repos.clone()),
            account: AccountService::new(repos.clone()),
        }
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub pool: PgPool,
    pub repos: Repos,
    pub services: Services,
}

impl AppContext {
    pub fn new(pool: PgPool) -> Self {
        let repos = Repos::new(pool.clone());
        let services = Services::new(repos.clone());
        Self {
            pool,
            repos,
            services,
        }
    }
}
