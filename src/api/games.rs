use crate::db;
use crate::result::Result;
use askama::Template;
use rocket::{get, post, State};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "game_overview.html")]
pub struct GameOverview {
    games: Vec<String>,
}

impl<'c> GameOverview {
    async fn new(executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>) -> Result<Self> {
        Ok(Self {
            games: db::games::get_all(executor).await?,
        })
    }
}

#[post("/games/<name>")]
pub async fn post(pool: &State<PgPool>, name: &str) -> Result<GameOverview> {
    let mut tx = pool.begin().await?;
    db::games::create(&mut *tx, name).await?;
    let overview = GameOverview::new(&mut *tx).await?;
    tx.commit().await?;
    Ok(overview)
}

#[get("/games")]
pub async fn get(pool: &State<PgPool>) -> Result<GameOverview> {
    GameOverview::new(&**pool).await
}
