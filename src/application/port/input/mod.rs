mod send_money_usecase;

pub use send_money_usecase::*;

use shaku::Interface;

pub trait PingPongUseCase: Interface {
    fn pong(&self, ping: String) -> String;
}

pub struct TestCommand {
    pub data: String,
}

impl TestCommand {
    pub fn try_new(data: String) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Must not be empty".into());
        }

        let cmd = Self { data };
        Ok(cmd)
    }
}

#[rocket::async_trait]
pub trait HelloWorldUseCase: Interface {
    async fn hello_world(&self) -> String;
}
