use crate::error::AppError;
use crate::model::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(
        &self,
        uid: Uuid,
        account_id: Uuid,
        created_at: &chrono::DateTime<chrono::Utc>,
        updated_at: &chrono::DateTime<chrono::Utc>,
        input: &CreateTaskRequest,
    ) -> Result<Task, AppError> {
        let task = sqlx::query_as!(
            Task,
            r#"INSERT INTO task(id, account_id, title, description, type, subject, status, due_date, completed_at, source, created_at, created_by, updated_at, updated_by, is_deleted)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
               RETURNING id, account_id, title, description,
                         type AS task_type,
                         subject, status, due_date, completed_at, source,
                         created_at, created_by, updated_at, updated_by, is_deleted"#,
            uid,
            account_id,
            input.title,
            input.description,
            input.task_type,
            input.subject,
            "active",
            input.due_date,
            None::<chrono::DateTime<chrono::Utc>>,
            "manual",
            created_at,
            account_id,
            updated_at,
            account_id,
            false,
        ).fetch_one(&self.pool).await?;

        Ok(task)
    }

    pub async fn find_by_id_and_account(
        &self,
        task_id: Uuid,
        account_id: Uuid,
    ) -> Result<Task, AppError> {
        let task = sqlx::query_as!(
            Task,
            r#"
                SELECT id, account_id, title, description,
                    type AS task_type,
                    subject, status, due_date, completed_at, source,
                    created_at, created_by, updated_at, updated_by, is_deleted
                FROM task
                WHERE id = $1 AND account_id = $2 AND is_deleted = false
            "#,
            task_id,
            account_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    pub async fn find_all_by_account(&self, account_id: Uuid) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as!(
            Task,
            r#"SELECT id, account_id, title, description,
                    type AS task_type,
                    subject, status, due_date, completed_at, source,
                    created_at, created_by, updated_at, updated_by, is_deleted
             FROM task
             WHERE account_id = $1 AND is_deleted = false
             ORDER BY created_at DESC
        "#,
            account_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    pub async fn update(
        &self,
        task_id: Uuid,
        account_id: Uuid,
        input: UpdateTaskRequest,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now();
        sqlx::query!(
            r#"
            UPDATE task SET
              title = COALESCE($1, title),
              description = COALESCE($2, description),
              type = COALESCE($3, type),
              subject = COALESCE($4, subject),
              status = COALESCE($5, status),
              due_date = COALESCE($6, due_date),
              updated_at = $7,
              updated_by = $8
          WHERE id = $9 AND account_id = $10
        "#,
            input.title,
            input.description,
            input.task_type,
            input.subject,
            input.status,
            input.due_date,
            now,
            account_id,
            task_id,
            account_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, task_id: Uuid, account_id: Uuid) -> Result<(), AppError> {
        let now = chrono::Utc::now();
        sqlx::query!(
            r#"
                UPDATE task SET is_deleted = true, updated_at=$1, updated_by=$2 where id = $3 AND account_id = $4
            "#,
            now,
            account_id,
            task_id,
            account_id
        ).execute(&self.pool).await?;

        Ok(())
    }
}
