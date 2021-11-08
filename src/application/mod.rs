pub mod port;
mod send_money_service;

pub use send_money_service::*;

use port::input::HelloWorldUseCase;
use port::input::PingPongUseCase;

#[derive(Component)]
#[shaku(interface = PingPongUseCase)]
pub struct PingPongUseCaseImpl;

impl PingPongUseCase for PingPongUseCaseImpl {
    fn pong(&self, ping: String) -> String {
        ping
    }
}

#[derive(Component)]
#[shaku(interface = HelloWorldUseCase)]
pub struct HelloWorldUseCaseImpl;

#[rocket::async_trait]
impl HelloWorldUseCase for HelloWorldUseCaseImpl {
    async fn hello_world(&self) -> String {
        "Hello World!".to_owned()
    }
}
