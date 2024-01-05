use rocket::{fs::FileServer, get, post, routes, State};
use sqlx::PgPool;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/clicked")]
async fn clicked(pool: &State<DBPool>) -> String {
    sqlx::query("INSERT INTO clicks VALUES (CURRENT_TIMESTAMP)")
        .execute(&pool.0)
        .await
        .expect("db failed");
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM clicks")
        .fetch_one(&pool.0)
        .await
        .expect("db failed 2");
    count.count.expect("clicks failed").to_string()
}

struct DBPool(PgPool);

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .expect("Migration failed");
    let rocket = rocket::build()
        .mount("/api", routes![index, clicked])
        .mount("/", FileServer::from("static/"))
        .manage(DBPool(pool));

    Ok(rocket.into())
}
