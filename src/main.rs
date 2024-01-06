mod result;

use rocket::{fs::FileServer, post, routes, State};
use sqlx::PgPool;

#[derive(thiserror::Error, Debug)]
struct NotFoundError;

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No value found")
    }
}

#[post("/up")]
async fn up(pool: &State<DBPool>) -> result::Result<String> {
    let mut tx = pool.0.begin().await?;
    sqlx::query("INSERT INTO clicks VALUES (CURRENT_TIMESTAMP)")
        .execute(&mut *tx)
        .await?;
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM clicks")
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(count.count.map(|x| x.to_string()).ok_or(NotFoundError)?)
}

#[post("/down")]
async fn down(pool: &State<DBPool>) -> String {
    sqlx::query("DELETE FROM clicks WHERE date IN (SELECT MAX(date) FROM clicks)")
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
#[allow(clippy::no_effect_underscore_binding)]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    let rocket = rocket::build()
        .mount("/api", routes![up, down])
        .mount("/", FileServer::from("static/"))
        .manage(DBPool(pool));

    Ok(rocket.into())
}
