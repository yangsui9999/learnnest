use crate::app::from_pool::FromPool;
use crate::error::AppError;
use crate::model::account::{Account, AccountCreate};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AccountRepository {
    pool: PgPool,
}

impl FromPool for AccountRepository {
    fn from_pool(pool: PgPool) -> Self {
        Self::new(pool)
    }
}

impl AccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, account: AccountCreate) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO account(id, username, password_hash, nickname, role, created_at, created_by, updated_at, updated_by, is_deleted)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            account.id,
            account.username,
            account.password_hash,
            account.nickname,
            account.role,
            account.created_at,
            account.created_by,
            account.updated_at,
            account.updated_by,
            account.is_deleted,
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Account, AppError> {
        let account = sqlx::query_as!(
            Account,
            r#"select * from account where username = $1"#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }
}
