use crate::common::from_pool::FromPool;
use crate::repository::task::TaskRepository;
use crate::service::task::TaskService;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Repos {
    pub task: TaskRepository,
    // 其它的 repo
}
impl Repos {
    pub fn new(pool: PgPool) -> Self {
        Self {
            task: TaskRepository::from_pool(pool),
        }
    }
}

#[derive(Clone)]
pub struct Services {
    pub task: TaskService,
    // 其它的 service
}

impl Services {
    pub fn new(repos: Repos) -> Self {
        Self {
            task: TaskService::new(repos),
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
