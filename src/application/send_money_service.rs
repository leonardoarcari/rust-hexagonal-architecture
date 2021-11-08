use anyhow::Result;

use super::port::input::{SendMoneyCommand, SendMoneyUseCase};

#[derive(Component)]
#[shaku(interface = SendMoneyUseCase)]
pub struct SendMoneyService;

#[rocket::async_trait]
impl SendMoneyUseCase for SendMoneyService {
    async fn send_money(&self, _cmd: SendMoneyCommand) -> Result<bool> {
        todo!()
    }
}
