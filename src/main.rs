mod api;
mod error;
mod result;

use error::Error;
use result::Result;

use rocket::{get, post, routes, State};
use sqlx::PgPool;

#[post("/up")]
async fn up(pool: &State<PgPool>) -> Result<String> {
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO clicks VALUES (CURRENT_TIMESTAMP)")
        .execute(&mut *tx)
        .await?;
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM clicks")
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(count.count.ok_or(Error::NotFoundError)?.to_string())
}

#[post("/down")]
async fn down(pool: &State<PgPool>) -> Result<String> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM clicks WHERE date IN (SELECT MAX(date) FROM clicks)")
        .execute(&mut *tx)
        .await?;
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM clicks")
        .fetch_one(&mut *tx)
        .await?;
    Ok(count.count.ok_or(Error::NotFoundError)?.to_string())
}

#[get("/count")]
async fn count(pool: &State<PgPool>) -> Result<String> {
    let mut tx = pool.begin().await?;
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM clicks")
        .fetch_one(&mut *tx)
        .await?;
    Ok(count.count.ok_or(Error::NotFoundError)?.to_string())
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate
}

#[shuttle_runtime::main]
#[allow(clippy::no_effect_underscore_binding)]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    let rocket = rocket::build()
        .mount("/api", routes![up, down, count, api::matches::post])
        .mount("/", routes![index])
        .manage(pool);

    Ok(rocket.into())
}
