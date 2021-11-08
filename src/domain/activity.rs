use chrono::{DateTime, Utc};

use super::{account::AccountId, money::Money};

#[derive(Debug)]
pub struct ActivityWindow {
    activities: Vec<Activity>,
}

impl ActivityWindow {
    pub fn new(activities: Vec<Activity>) -> Self {
        // if activities.is_empty() {
        //     panic!("Unexpected empty activities vector");
        // }

        Self { activities }
    }

    /// Get a reference to the activity window's activities.
    pub fn activities(&self) -> &[Activity] {
        self.activities.as_slice()
    }

    pub fn add_activity(&mut self, a: Activity) {
        self.activities.push(a)
    }

    pub fn get_start_timestamp(&self) -> DateTime<Utc> {
        self.activities
            .iter()
            .min_by_key(|a| a.timestamp())
            .expect("Unexpect empty activities vector")
            .timestamp()
            .clone()
    }

    pub fn get_end_timestamp(&self) -> DateTime<Utc> {
        self.activities
            .iter()
            .max_by_key(|a| a.timestamp())
            .expect("Unexpected empty activities vector")
            .timestamp()
            .clone()
    }

    pub fn calculate_balance(&self, id: AccountId) -> Money {
        let deposit_balance = self
            .activities
            .iter()
            .filter(|a| a.target_account_id() == &id)
            .map(Activity::money)
            .fold(Money(0), |acc, m| acc + *m);

        let withdrawal_balance = self
            .activities
            .iter()
            .filter(|a| a.source_account_id() == &id)
            .map(Activity::money)
            .fold(Money(0), |acc, m| acc + *m);

        deposit_balance - withdrawal_balance
    }
}

#[derive(Debug, Clone, Builder)]
pub struct Activity {
    #[builder(default = "None")]
    id: Option<ActivityId>,
    #[builder(default = "self.default_owner_account()?")]
    owner_account_id: AccountId,
    source_account_id: AccountId,
    target_account_id: AccountId,
    #[builder(default = "Utc::now()")]
    timestamp: DateTime<Utc>,
    money: Money,
}

impl ActivityBuilder {
    fn default_owner_account(&self) -> Result<AccountId, String> {
        match self.source_account_id {
            Some(ref id) => Ok(*id),
            None => Err("Source account id is missing".into()),
        }
    }
}

impl Activity {
    pub fn new(
        owner_account_id: AccountId,
        source_account_id: AccountId,
        target_account_id: AccountId,
        timestamp: DateTime<Utc>,
        money: Money,
    ) -> Self {
        Self {
            id: None,
            owner_account_id,
            source_account_id,
            target_account_id,
            timestamp,
            money,
        }
    }

    pub fn new_with_id(
        id: ActivityId,
        owner_account_id: AccountId,
        source_account_id: AccountId,
        target_account_id: AccountId,
        timestamp: DateTime<Utc>,
        money: Money,
    ) -> Self {
        Self {
            id: Some(id),
            owner_account_id,
            source_account_id,
            target_account_id,
            timestamp,
            money,
        }
    }

    pub fn id(&self) -> Option<&ActivityId> {
        self.id.as_ref()
    }

    /// Get a reference to the activity's owner account id.
    pub fn owner_account_id(&self) -> &AccountId {
        &self.owner_account_id
    }

    /// Get a reference to the activity's source account id.
    pub fn source_account_id(&self) -> &AccountId {
        &self.source_account_id
    }

    /// Get a reference to the activity's target account id.
    pub fn target_account_id(&self) -> &AccountId {
        &self.target_account_id
    }

    /// Get a reference to the activity's timestamp.
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Get a reference to the activity's money.
    pub fn money(&self) -> &Money {
        &self.money
    }

    pub fn with_timestamp(self, ts: DateTime<Utc>) -> Activity {
        match self.id {
            Some(id) => Activity::new_with_id(
                id,
                self.owner_account_id,
                self.source_account_id,
                self.target_account_id,
                ts,
                self.money,
            ),
            None => Activity::new(
                self.owner_account_id,
                self.source_account_id,
                self.target_account_id,
                ts,
                self.money,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActivityId(pub u64);

#[cfg(test)]
pub mod tests {
    use super::*;

    use chrono::prelude::*;

    #[test]
    fn activity_window_calculates_start_timestamp() {
        // Given
        let window = ActivityWindow::new(vec![
            default_activity().timestamp(start_date()).build().unwrap(),
            default_activity()
                .timestamp(in_between_date())
                .build()
                .unwrap(),
            default_activity().timestamp(end_date()).build().unwrap(),
        ]);

        // Expect
        assert_eq!(window.get_start_timestamp(), start_date());
    }

    #[test]
    fn activity_window_calculates_end_timestamp() {
        // Given
        let window = ActivityWindow::new(vec![
            default_activity().timestamp(start_date()).build().unwrap(),
            default_activity()
                .timestamp(in_between_date())
                .build()
                .unwrap(),
            default_activity().timestamp(end_date()).build().unwrap(),
        ]);

        // Expect
        assert_eq!(window.get_end_timestamp(), end_date());
    }

    #[test]
    fn calculates_balance() {
        // Given
        let account1 = AccountId(1);
        let account2 = AccountId(2);
        let window = ActivityWindow::new(vec![
            ActivityBuilder::default()
                .source_account_id(account1)
                .target_account_id(account2)
                .money(Money(999))
                .build()
                .unwrap(),
            ActivityBuilder::default()
                .source_account_id(account1)
                .target_account_id(account2)
                .money(Money(1))
                .build()
                .unwrap(),
            ActivityBuilder::default()
                .source_account_id(account2)
                .target_account_id(account1)
                .money(Money(500))
                .build()
                .unwrap(),
        ]);

        // Expect
        assert_eq!(window.calculate_balance(account1), Money(-500));
        assert_eq!(window.calculate_balance(account2), Money(500));
    }

    pub fn default_activity() -> ActivityBuilder {
        ActivityBuilder::default()
            .source_account_id(AccountId(42))
            .target_account_id(AccountId(41))
            .money(Money(999))
            .clone()
    }

    fn start_date() -> DateTime<Utc> {
        Utc.ymd(2019, 8, 3).and_hms(0, 0, 0)
    }

    fn end_date() -> DateTime<Utc> {
        Utc.ymd(2019, 8, 5).and_hms(0, 0, 0)
    }

    fn in_between_date() -> DateTime<Utc> {
        Utc.ymd(2019, 8, 4).and_hms(0, 0, 0)
    }
}
