use crate::result::Result;

pub async fn create<'c>(
    executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
    name: &str,
) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO game (name)
        VALUES ($1)",
        name,
    )
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn get_all<'c>(
    executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
) -> Result<Vec<String>> {
    Ok(sqlx::query!("SELECT name FROM game")
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(|x| x.name)
        .collect())
}
