use std::{convert::TryInto, sync::Arc};

use anyhow::Result;
use chrono::{DateTime, Utc};
use num_traits::ToPrimitive;
use sqlx::types::BigDecimal;

use crate::{
    application::port::output::LoadAccountPort,
    domain::{
        account::{Account, AccountBuilder, AccountId},
        activity::{Activity, ActivityBuilder, ActivityId, ActivityWindow},
        money::Money,
    },
    infrastructure::db::DataSource,
};

#[derive(Component)]
#[shaku(interface = LoadAccountPort)]
pub struct AccountRepository {
    #[shaku(inject)]
    pool: Arc<dyn DataSource>,
}

#[rocket::async_trait]
impl LoadAccountPort for AccountRepository {
    async fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: DateTime<Utc>,
    ) -> Result<Account> {
        // Account must exist
        let _ = self.find_account(account_id).await?;

        let activities_dto = self
            .load_activities_by_owner_since(account_id, baseline_date)
            .await?;

        log::info!("activities_dto: {:?}", activities_dto);

        let withdrawal_balance = self
            .get_withdrawal_balance_until(account_id, baseline_date)
            .await?
            .unwrap_or(0);

        let deposit_balance = self
            .get_deposit_balance_until(account_id, baseline_date)
            .await?
            .unwrap_or(0);

        let baseline_balance = Money(deposit_balance) - Money(withdrawal_balance);

        let activities = activities_dto
            .into_iter()
            .map(|dto| dto.try_into())
            .collect::<Result<Vec<Activity>>>()?;

        let account = AccountBuilder::default()
            .id(account_id)
            .baseline_balance(baseline_balance)
            .activity_window(ActivityWindow::new(activities))
            .build()?;

        Ok(account)
    }
}

impl AccountRepository {
    pub async fn find_account(&self, id: AccountId) -> Result<AccountDto> {
        let account: AccountDto = sqlx::query_as("SELECT id FROM account WHERE id = $1")
            .bind(id.0 as i64)
            .fetch_one(self.pool.get())
            .await?;

        Ok(account)
    }

    pub async fn load_activities_by_owner_since(
        &self,
        id: AccountId,
        baseline_date: DateTime<Utc>,
    ) -> Result<Vec<ActivityDto>> {
        let activities: Vec<ActivityDto> = sqlx::query_as(
            r#"
            SELECT
                a.id, a.timestamp, a.owner_account_id, a.source_account_id, a.target_account_id, a.amount
            FROM
                activity a
            WHERE
                a.owner_account_id = $1
            AND a.timestamp >= $2
            "#,
        )
        .bind(id.0 as i64)
        .bind(baseline_date)
        .fetch_all(self.pool.get())
        .await?;

        Ok(activities)
    }

    pub async fn get_withdrawal_balance_until(
        &self,
        id: AccountId,
        baseline_date: DateTime<Utc>,
    ) -> Result<Option<i64>> {
        let amount: Option<(BigDecimal,)> = sqlx::query_as(
            r#"
            SELECT coalesce(sum(a.amount), 0.0)
            FROM
                activity a
            WHERE
                a.source_account_id = $1
            AND a.owner_account_id = $2
            AND a.timestamp < $3
            "#,
        )
        .bind(id.0 as i64)
        .bind(id.0 as i64)
        .bind(baseline_date)
        .fetch_optional(self.pool.get())
        .await?;

        Ok(amount.and_then(|(a,)| a.to_i64()))
    }

    pub async fn get_deposit_balance_until(
        &self,
        id: AccountId,
        baseline_date: DateTime<Utc>,
    ) -> Result<Option<i64>> {
        let amount: Option<(BigDecimal,)> = sqlx::query_as(
            r#"
            SELECT coalesce(sum(a.amount), 0.0)
            FROM
                activity a
            WHERE
                a.target_account_id = $1
            AND a.owner_account_id = $2
            AND a.timestamp < $3
            "#,
        )
        .bind(id.0 as i64)
        .bind(id.0 as i64)
        .bind(baseline_date)
        .fetch_optional(self.pool.get())
        .await?;

        Ok(amount.and_then(|(a,)| a.to_i64()))
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct AccountDto {
    id: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ActivityDto {
    id: i64,
    timestamp: DateTime<Utc>,
    owner_account_id: i64,
    source_account_id: i64,
    target_account_id: i64,
    amount: i64,
}

impl TryInto<Activity> for ActivityDto {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Activity, Self::Error> {
        Ok(ActivityBuilder::default()
            .id(Some(ActivityId(self.id as u64)))
            .owner_account_id(AccountId(self.owner_account_id as u64))
            .source_account_id(AccountId(self.source_account_id as u64))
            .target_account_id(AccountId(self.target_account_id as u64))
            .timestamp(self.timestamp)
            .money(Money(self.amount))
            .build()?)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use rocket::tokio;
    use shaku::HasComponent;

    use crate::infrastructure::tests::{self, testing_module};

    use super::*;

    #[tokio::test]
    async fn it_loads_account() -> Result<()> {
        // Init
        tests::setup();
        let module = testing_module().await.build();
        let port: &dyn LoadAccountPort = module.resolve_ref();

        // Given
        let account_id = AccountId(1);
        let baseline_date = Utc.ymd(2018, 8, 10).and_hms(0, 0, 0);

        // When
        let account = port.load_account(account_id, baseline_date).await?;
        let n_activities = account.activity_window().activities().len();
        let balance = account.calculate_balance();

        // Expect
        assert_eq!(n_activities, 2);
        assert_eq!(balance, Money(500));
        Ok(())
    }
}
