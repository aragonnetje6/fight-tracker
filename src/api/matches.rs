use crate::db::matches::{create_match, delete_last, get_match_overview, MatchResult};
use crate::result::Result;
use rocket::{delete, post, State};
use sqlx::PgPool;
use std::fmt::Write;

#[post("/matches/<game>/<character>/<result>")]
pub async fn post(
    pool: &State<PgPool>,
    game: &str,
    character: &str,
    result: MatchResult,
) -> Result<String> {
    let mut tx = pool.begin().await?;
    create_match(&mut *tx, game, character, result).await?;
    let mut count = String::new();
    get_match_overview(&mut *tx, game, character)
        .await?
        .into_iter()
        .fold(&mut count, |acc, x| {
            write!(acc, r"<tr><td>{}</td><td>{}</td></tr>", x.result, x.count).expect("fuck off");
            acc
        });
    tx.commit().await?;
    Ok(format!(r"<table>{count}</table>"))
}

#[delete("/matches/<game>/<character>")]
pub async fn delete(pool: &State<PgPool>, game: &str, character: &str) -> Result<String> {
    let mut tx = pool.begin().await?;
    delete_last(&mut *tx, game, character).await?;
    let mut count = String::new();
    get_match_overview(&mut *tx, game, character)
        .await?
        .into_iter()
        .fold(&mut count, |acc, x| {
            write!(acc, r"<tr><td>{}</td><td>{}</td></tr>", x.result, x.count).expect("fuck off");
            acc
        });
    tx.commit().await?;
    Ok(format!(r"<table>{count}</table>"))
}
