mod api;
mod db;
mod error;
mod result;

use api::games::GameOverview;
use rocket::{get, routes, State};
use sqlx::PgPool;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    games: GameOverview,
}

impl<'c> IndexTemplate {
    async fn new(
        executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
    ) -> result::Result<Self> {
        Ok(Self {
            games: GameOverview::new(executor).await?,
        })
    }
}

#[get("/")]
async fn index(pool: &State<PgPool>) -> result::Result<IndexTemplate> {
    IndexTemplate::new(&**pool).await
}

#[derive(askama::Template)]
#[template(path = "add_game.html")]
struct AddGameTemplate;

#[get("/add_game")]
fn add_game() -> AddGameTemplate {
    AddGameTemplate
}

#[derive(askama::Template)]
#[template(path = "add_character.html")]
struct AddCharacterTemplate {
    games: Vec<Game>,
}

impl<'c> AddCharacterTemplate {
    async fn new(
        executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
        selected_game: Option<&str>,
    ) -> result::Result<Self> {
        Ok(AddCharacterTemplate {
            games: db::games::get_all(executor)
                .await?
                .into_iter()
                .map(|name| {
                    let selected = selected_game.is_some_and(|g| g == name);
                    Game { name, selected }
                })
                .collect(),
        })
    }
}

struct Game {
    name: String,
    selected: bool,
}

#[get("/add_character?<game>")]
async fn add_character(
    game: Option<&str>,
    pool: &State<PgPool>,
) -> result::Result<AddCharacterTemplate> {
    AddCharacterTemplate::new(&**pool, game).await
}

#[shuttle_runtime::main]
#[allow(clippy::no_effect_underscore_binding)]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    let rocket = rocket::build()
        .mount(
            "/api",
            routes![
                api::matches::post,
                api::matches::delete,
                api::matches::get,
                api::games::get,
                api::games::post,
                api::characters::get,
                api::characters::post,
            ],
        )
        .mount("/", routes![index, add_game, add_character])
        .manage(pool);

    Ok(rocket.into())
}
