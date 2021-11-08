use anyhow::Result;
use shaku::Interface;

use crate::domain::{account::AccountId, money::Money};

#[rocket::async_trait]
pub trait SendMoneyUseCase: Interface {
    async fn send_money(&self, cmd: SendMoneyCommand) -> Result<bool>;
}

pub struct SendMoneyCommand {
    source_account_id: AccountId,
    target_account_id: AccountId,
    money: Money,
}

impl SendMoneyCommand {
    pub async fn try_new(
        source_account_id: AccountId,
        target_account_id: AccountId,
        money: Money,
    ) -> Result<Self> {
        Ok(Self {
            source_account_id,
            target_account_id,
            money,
        })
    }

    /// Get a reference to the send money command's source account id.
    pub fn source_account_id(&self) -> &AccountId {
        &self.source_account_id
    }

    /// Get a reference to the send money command's target account id.
    pub fn target_account_id(&self) -> &AccountId {
        &self.target_account_id
    }

    /// Get a reference to the send money command's money.
    pub fn money(&self) -> &Money {
        &self.money
    }
}
