use crate::db::matches::{create_match, MatchResult};
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
    sqlx::query!(
        "SELECT match.result AS \"result: MatchResult\", COUNT(*) AS \"count!\"
        FROM match
        JOIN character ON character.id = match.character_id
        JOIN game ON character.game_id = game.id
        WHERE game.name = $1 
        AND character.name = $2
        GROUP BY match.result",
        game,
        character
    )
    .fetch_all(&mut *tx)
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
    sqlx::query!(
        "DELETE FROM match
        WHERE id = (
            SELECT max(match.id)
            FROM match
            JOIN character ON match.character_id = character.id
            JOIN game ON character.game_id = game.id
            WHERE game.name = $1
                AND character.name = $2
        )",
        game,
        character,
    )
    .execute(&mut *tx)
    .await?;
    let mut count = String::new();
    sqlx::query!(
        "SELECT match.result AS \"result: MatchResult\", COUNT(*) AS \"count!\"
        FROM match
        JOIN character ON character.id = match.character_id
        JOIN game ON character.game_id = game.id
        WHERE game.name = $1 
        AND character.name = $2
        GROUP BY match.result",
        game,
        character
    )
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .fold(&mut count, |acc, x| {
        write!(acc, r"<tr><td>{}</td><td>{}</td></tr>", x.result, x.count).expect("fuck off");
        acc
    });
    tx.commit().await?;
    Ok(format!(r"<table>{count}</table>"))
}
