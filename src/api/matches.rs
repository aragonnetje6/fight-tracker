use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use super::{Error, Result};
use rocket::{post, request::FromParam, State};
use sqlx::PgPool;

#[derive(Debug, sqlx::Type, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[sqlx(type_name = "matchresult")]
#[sqlx(rename_all = "lowercase")]
pub enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl<'a> FromParam<'a> for MatchResult {
    type Error = Error;

    fn from_param(param: &'a str) -> std::result::Result<Self, Self::Error> {
        param.parse()
    }
}

impl FromStr for MatchResult {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "win" => Ok(Self::Win),
            "loss" => Ok(Self::Loss),
            "draw" => Ok(Self::Draw),
            _ => Err(Error::ParseError),
        }
    }
}

impl Display for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Win => "Win",
                Self::Loss => "Loss",
                Self::Draw => "Draw",
            }
        )
    }
}

#[post("/matches/<game>/<character>/<result>")]
pub async fn post(
    pool: &State<PgPool>,
    game: &str,
    character: &str,
    result: MatchResult,
) -> Result<String> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "INSERT INTO match (result, character_id, timestamp)
        VALUES ($1, (
            SELECT character.id
            FROM character
            JOIN game ON game.id = character.game_id
            WHERE character.name = $2
            AND game.name = $3
        ), CURRENT_TIMESTAMP)",
        result as MatchResult,
        character,
        game,
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
