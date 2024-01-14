use crate::db;
use crate::result::Result;
use askama::Template;
use rocket::{get, post, State};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "character_overview.html")]
pub struct CharacterOverview {
    characters: Vec<String>,
    game: String,
}

impl<'c> CharacterOverview {
    async fn new(
        executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
        game: &str,
    ) -> Result<Self> {
        Ok(Self {
            characters: db::characters::get(executor, game).await?,
            game: game.to_string(),
        })
    }
}

#[post("/characters/<game>/<character_name>")]
pub async fn post(
    pool: &State<PgPool>,
    game: &str,
    character_name: &str,
) -> Result<CharacterOverview> {
    let mut tx = pool.begin().await?;
    db::characters::create(&mut *tx, game, character_name).await?;
    let overview = CharacterOverview::new(&mut *tx, game).await?;
    tx.commit().await?;
    Ok(overview)
}

#[get("/characters/<game>")]
pub async fn get(pool: &State<PgPool>, game: &str) -> Result<CharacterOverview> {
    CharacterOverview::new(&**pool, game).await
}
