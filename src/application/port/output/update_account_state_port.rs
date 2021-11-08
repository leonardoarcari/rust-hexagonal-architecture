use anyhow::Result;

use shaku::Interface;

use crate::domain::account::Account;

#[rocket::async_trait]
pub trait UpdateAccountStatePort: Interface {
    async fn update_activities(&self, account: &Account) -> Result<Account>;
}
