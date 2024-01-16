mod api;
mod db;
mod error;
mod result;

use rocket::{get, routes};
use sqlx::PgPool;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate
}

#[derive(askama::Template)]
#[template(path = "add_game.html")]
struct AddGameTemplate;

#[get("/add_game")]
fn add_game() -> AddGameTemplate {
    AddGameTemplate
}

#[shuttle_runtime::main]
#[allow(clippy::no_effect_underscore_binding)]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    let rocket = rocket::build()
        .mount(
            "/api",
            routes![
                api::matches::post,
                api::matches::delete,
                api::matches::get,
                api::games::get,
                api::games::post,
                api::characters::get,
                api::characters::post,
            ],
        )
        .mount("/", routes![index, add_game])
        .manage(pool);

    Ok(rocket.into())
}
