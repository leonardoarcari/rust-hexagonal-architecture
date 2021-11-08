mod api;

use rocket::{fairing, Build, Rocket};

pub async fn configure_rest(rocket: Rocket<Build>) -> fairing::Result {
    let rocket = rocket
        .mount("/hello", rocket::routes![api::world])
        .mount("/", rocket::routes![api::ping]);

    Ok(rocket)
}
