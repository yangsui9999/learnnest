use crate::app::context::Repos;
use crate::error::AppError;
use crate::model::account::{Account, AccountCreate};

#[derive(Clone)]
pub struct AccountService {
    repos: Repos,
}
impl AccountService {
    pub fn new(repos: Repos) -> Self {
        Self { repos }
    }

    pub async fn get(&self, name: String) -> Result<Account, AppError> {
        self.repos.account.get_by_username(&name).await
    }

    pub async fn create(&self, account: AccountCreate) -> Result<(), AppError> {
        self.repos.account.insert(account).await
    }
}
