use rocket_hexagonal::{
    adapter::input::rest,
    infrastructure::container::{connect_db, default_module},
};

#[rocket::launch]
async fn rocket() -> _ {
    let uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        "azueljos", "azulejos-pg-pwd", "localhost", 5432, "azulejos"
    );
    let db_pool = connect_db(&uri).await;

    rocket::build()
        .manage(Box::new(default_module(db_pool).await.build()))
        .attach(rocket::fairing::AdHoc::try_on_ignite(
            "REST Adapter",
            rest::configure_rest,
        ))
}
