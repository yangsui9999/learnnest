use crate::app::context::Repos;
use crate::error::AppError;
use crate::model::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskService {
    repos: Repos,
}

impl TaskService {
    pub fn new(repos: Repos) -> Self {
        Self { repos }
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        input: &CreateTaskRequest,
    ) -> Result<Task, AppError> {
        let uid = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();

        self.repos
            .task
            .insert(uid, account_id, &now, &now, input)
            .await
    }

    pub async fn get(&self, task_id: Uuid, account_id: Uuid) -> Result<Task, AppError> {
        self.repos
            .task
            .find_by_id_and_account(task_id, account_id)
            .await
    }

    pub async fn list(&self, account_id: Uuid) -> Result<Vec<Task>, AppError> {
        self.repos.task.find_all_by_account(account_id).await
    }

    pub async fn update(
        &self,
        task_id: Uuid,
        account_id: Uuid,
        input: UpdateTaskRequest,
    ) -> Result<(), AppError> {
        self.repos.task.update(task_id, account_id, input).await
    }

    pub async fn delete(&self, task_id: Uuid, account_id: Uuid) -> Result<(), AppError> {
        self.repos.task.delete(task_id, account_id).await
    }
}
