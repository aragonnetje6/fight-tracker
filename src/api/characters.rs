use crate::db;
use crate::result::Result;
use askama::Template;
use rocket::{form::Form, get, post, FromForm, State};
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

#[derive(Template)]
#[template(path = "add_character_success.html")]
pub struct AddCharacterSuccess {
    normal: crate::AddCharacterTemplate,
}

#[derive(FromForm)]
pub struct CharacterForm<'a> {
    game: &'a str,
    character_name: &'a str,
}

#[post("/characters", data = "<character_form>")]
pub async fn post(
    pool: &State<PgPool>,
    character_form: Form<CharacterForm<'_>>,
) -> Result<AddCharacterSuccess> {
    db::characters::create(&**pool, character_form.game, character_form.character_name).await?;
    Ok(AddCharacterSuccess {
        normal: crate::AddCharacterTemplate::new(&**pool, Some(character_form.game)).await?,
    })
}

#[get("/characters/<game>")]
pub async fn get(pool: &State<PgPool>, game: &str) -> Result<CharacterOverview> {
    CharacterOverview::new(&**pool, game).await
}
