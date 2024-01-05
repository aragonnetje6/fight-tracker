use rocket::{fs::FileServer, get, routes};
use sqlx::PgPool;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

struct DBPool(PgPool);

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/api", routes![index])
        .mount("/", FileServer::from("static/"))
        .manage(DBPool(pool));

    Ok(rocket.into())
}
