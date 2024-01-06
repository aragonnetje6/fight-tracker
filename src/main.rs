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
async fn up(pool: &State<PgPool>) -> result::Result<String> {
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO clicks VALUES (CURRENT_TIMESTAMP)")
        .execute(&mut *tx)
        .await?;
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM clicks")
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(count.count.ok_or(NotFoundError)?.to_string())
}

#[post("/down")]
async fn down(pool: &State<PgPool>) -> result::Result<String> {
    sqlx::query("DELETE FROM clicks WHERE date IN (SELECT MAX(date) FROM clicks)")
        .execute(&**pool)
        .await?;
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM clicks")
        .fetch_one(&**pool)
        .await?;
    Ok(count.count.ok_or(NotFoundError)?.to_string())
}

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
        .manage(pool);

    Ok(rocket.into())
}
