use crate::error::AppError;
use crate::model::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::repository::task::TaskRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskService {
    task_repo: TaskRepository,
}

impl TaskService {
    pub fn new(pool: PgPool) -> Self {
        let task_repo = TaskRepository::new(pool);
        Self { task_repo }
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        input: &CreateTaskRequest,
    ) -> Result<Task, AppError> {
        let uid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();

        self.task_repo
            .insert(uid, account_id, &now, &now, input)
            .await
    }

    pub async fn get(&self, task_id: Uuid, account_id: Uuid) -> Result<Task, AppError> {
        self.task_repo
            .find_by_id_and_account(task_id, account_id)
            .await
    }

    pub async fn list(&self, account_id: Uuid) -> Result<Vec<Task>, AppError> {
        self.task_repo.find_all_by_account(account_id).await
    }

    pub async fn update(
        &self,
        task_id: Uuid,
        account_id: Uuid,
        input: UpdateTaskRequest,
    ) -> Result<(), AppError> {
        self.task_repo.update(task_id, account_id, input).await
    }

    pub async fn delete(&self, task_id: Uuid, account_id: Uuid) -> Result<(), AppError> {
        self.task_repo.delete(task_id, account_id).await
    }
}
