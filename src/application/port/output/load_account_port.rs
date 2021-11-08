use anyhow::Result;
use chrono::{DateTime, Utc};
use shaku::Interface;

use crate::domain::account::{Account, AccountId};

#[rocket::async_trait]
pub trait LoadAccountPort: Interface {
    async fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: DateTime<Utc>,
    ) -> Result<Account>;
}
