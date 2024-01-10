use crate::db::matches::{create_match, delete_last, get_match_overview, MatchResult};
use crate::result::Result;
use askama::Template;
use rocket::{delete, get, post, State};
use sqlx::PgPool;
use std::collections::BTreeMap;

#[derive(Template)]
#[template(path = "match_results.html")]
pub struct MatchOverview<'a> {
    game: &'a str,
    character: &'a str,
    wins: i64,
    draws: i64,
    losses: i64,
}

impl<'a, 'c> MatchOverview<'a> {
    async fn new(
        executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
        game: &'a str,
        character: &'a str,
    ) -> Result<Self> {
        let counts = get_match_overview(executor, game, character)
            .await?
            .into_iter()
            .map(|x| (x.result, x.count))
            .collect::<BTreeMap<MatchResult, i64>>();
        let wins = *counts.get(&MatchResult::Win).unwrap_or(&0);
        let draws = *counts.get(&MatchResult::Draw).unwrap_or(&0);
        let losses = *counts.get(&MatchResult::Loss).unwrap_or(&0);
        Ok(MatchOverview {
            game,
            character,
            wins,
            draws,
            losses,
        })
    }
}

#[post("/matches/<game>/<character>/<result>")]
pub async fn post<'a>(
    pool: &'a State<PgPool>,
    game: &'a str,
    character: &'a str,
    result: MatchResult,
) -> Result<MatchOverview<'a>> {
    let mut tx = pool.begin().await?;
    create_match(&mut *tx, game, character, result).await?;
    let overview = MatchOverview::new(&mut *tx, game, character).await?;
    tx.commit().await?;
    Ok(overview)
}

#[delete("/matches/<game>/<character>")]
pub async fn delete<'a>(
    pool: &'a State<PgPool>,
    game: &'a str,
    character: &'a str,
) -> Result<MatchOverview<'a>> {
    let mut tx = pool.begin().await?;
    delete_last(&mut *tx, game, character).await?;
    let overview = MatchOverview::new(&mut *tx, game, character).await?;
    tx.commit().await?;
    Ok(overview)
}

#[get("/matches/<game>/<character>")]
pub async fn get<'a>(
    pool: &'a State<PgPool>,
    game: &'a str,
    character: &'a str,
) -> Result<MatchOverview<'a>> {
    MatchOverview::new(&**pool, game, character).await
}
