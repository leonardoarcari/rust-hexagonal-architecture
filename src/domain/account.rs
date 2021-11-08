use super::{activity::ActivityBuilder, activity::ActivityWindow, money::Money};

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Account {
    #[builder(setter(strip_option))]
    id: Option<AccountId>,
    baseline_balance: Money,
    activity_window: ActivityWindow,
}

impl Account {
    pub fn new(baseline_balance: Money, activity_window: ActivityWindow) -> Self {
        Self {
            id: None,
            baseline_balance,
            activity_window,
        }
    }

    pub fn new_with_id(
        id: AccountId,
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> Self {
        Self {
            id: Some(id),
            baseline_balance,
            activity_window,
        }
    }

    /// Get a reference to the account's id.
    pub fn id(&self) -> Option<&AccountId> {
        self.id.as_ref()
    }

    pub fn calculate_balance(&self) -> Money {
        let activity_balance = self.activity_window.calculate_balance(
            self.id
                .expect("Cannot calculate balance. Account Id is not set."),
        );

        self.baseline_balance + activity_balance
    }

    pub fn withdraw(&mut self, money: Money, target_id: AccountId) -> bool {
        if !self.may_withdraw_money(money) {
            return false;
        }

        if let None = self.id {
            return false;
        }

        let id = self.id.as_ref().unwrap().clone();
        let withdrawal = ActivityBuilder::default()
            .source_account_id(id)
            .target_account_id(target_id)
            .money(money)
            .build()
            .unwrap();

        self.activity_window.add_activity(withdrawal);
        true
    }

    fn may_withdraw_money(&self, money: Money) -> bool {
        (self.calculate_balance() - money).is_positive_or_zero()
    }

    pub fn deposit(&mut self, money: Money, source_account: AccountId) -> bool {
        if let None = self.id {
            return false;
        }

        let id = self.id.as_ref().unwrap().clone();
        let deposit = ActivityBuilder::default()
            .owner_account_id(id)
            .source_account_id(source_account)
            .target_account_id(id)
            .money(money)
            .build()
            .unwrap();

        self.activity_window.add_activity(deposit);
        true
    }

    /// Get a reference to the account's baseline balance.
    pub fn baseline_balance(&self) -> &Money {
        &self.baseline_balance
    }

    /// Get a reference to the account's activity window.
    pub fn activity_window(&self) -> &ActivityWindow {
        &self.activity_window
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccountId(pub u64);

#[cfg(test)]
pub mod tests {
    use crate::domain::activity::tests::default_activity;

    use super::*;

    #[test]
    fn calculates_balance() {
        // Given
        let account_id = AccountId(1);
        let account = default_account()
            .id(account_id)
            .baseline_balance(Money(555))
            .activity_window(ActivityWindow::new(vec![
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(999))
                    .build()
                    .unwrap(),
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(1))
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        // When
        let balance = account.calculate_balance();

        // Expect
        assert_eq!(balance, Money(1555));
    }

    #[test]
    fn withdrawal_succeeds() {
        // Given
        let account_id = AccountId(1);
        let mut account = default_account()
            .id(account_id)
            .baseline_balance(Money(555))
            .activity_window(ActivityWindow::new(vec![
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(999))
                    .build()
                    .unwrap(),
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(1))
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        // When
        let success = account.withdraw(Money(555), AccountId(99));

        // Expect
        assert!(success);
        assert_eq!(3, account.activity_window().activities().len());
        assert_eq!(Money(1000), account.calculate_balance());
    }

    #[test]
    fn withdrawal_failure() {
        // Given
        let account_id = AccountId(1);
        let mut account = default_account()
            .id(account_id)
            .baseline_balance(Money(555))
            .activity_window(ActivityWindow::new(vec![
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(999))
                    .build()
                    .unwrap(),
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(1))
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        // When
        let success = account.withdraw(Money(1556), AccountId(99));

        // Expect
        assert!(!success);
        assert_eq!(2, account.activity_window().activities().len());
        assert_eq!(Money(1555), account.calculate_balance());
    }

    #[test]
    fn deposit_success() {
        // Given
        let account_id = AccountId(1);
        let mut account = default_account()
            .id(account_id)
            .baseline_balance(Money(555))
            .activity_window(ActivityWindow::new(vec![
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(999))
                    .build()
                    .unwrap(),
                default_activity()
                    .target_account_id(account_id)
                    .money(Money(1))
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        // When
        let success = account.deposit(Money(445), AccountId(99));

        // Expect
        assert!(success);
        assert_eq!(3, account.activity_window().activities().len());
        assert_eq!(Money(2000), account.calculate_balance());
    }

    pub fn default_account() -> AccountBuilder {
        AccountBuilder::default()
            .id(AccountId(42))
            .baseline_balance(Money(999))
            .activity_window(ActivityWindow::new(vec![
                default_activity().build().unwrap(),
                default_activity().build().unwrap(),
            ]))
    }
}
