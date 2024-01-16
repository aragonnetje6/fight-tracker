use crate::db;
use crate::result::Result;
use askama::Template;
use rocket::{form::Form, get, post, FromForm, State};
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

#[derive(Template)]
#[template(path = "add_game_success.html")]
pub struct AddGameSuccess;

#[derive(FromForm)]
pub struct GameForm<'a> {
    game_name: &'a str,
}

#[post("/games", data = "<game_form>")]
pub async fn post(pool: &State<PgPool>, game_form: Form<GameForm<'_>>) -> Result<AddGameSuccess> {
    db::games::create(&**pool, game_form.game_name).await?;
    Ok(AddGameSuccess)
}

#[get("/games")]
pub async fn get(pool: &State<PgPool>) -> Result<GameOverview> {
    GameOverview::new(&**pool).await
}
