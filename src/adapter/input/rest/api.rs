use crate::{
    application::port::input::{HelloWorldUseCase, PingPongUseCase},
    infrastructure::container::Inject,
};

#[rocket::get("/world")]
pub async fn world(hello_world_service: Inject<'_, dyn HelloWorldUseCase>) -> String {
    hello_world_service.hello_world().await
}

#[rocket::get("/ping/<message>")]
pub async fn ping(message: &str, ping_pong_service: Inject<'_, dyn PingPongUseCase>) -> String {
    ping_pong_service.pong(message.to_string())
}
