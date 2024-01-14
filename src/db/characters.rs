use crate::result::Result;

pub async fn create<'c>(
    executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
    game: &str,
    character_name: &str,
) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO character (name, game_id)
        VALUES ($1, (SELECT id FROM game WHERE name = $2))",
        character_name,
        game
    )
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn get<'c>(
    executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
    game: &str,
) -> Result<Vec<String>> {
    Ok(sqlx::query!(
        "SELECT character.name FROM character JOIN game ON game.id = character.game_id WHERE game.name = $1",
        game
    )
    .fetch_all(executor)
    .await?
    .into_iter()
    .map(|x| x.name)
    .collect())
}
