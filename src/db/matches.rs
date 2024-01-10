use std::{fmt::Display, str::FromStr};

use rocket::request::FromParam;

use crate::error::Error;
use crate::result::Result;

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

pub async fn create_match<'c>(
    exec: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
    game: &str,
    character: &str,
    match_result: MatchResult,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO match (result, character_id, timestamp)
        VALUES ($1, (
            SELECT character.id
            FROM character
            JOIN game ON game.id = character.game_id
            WHERE character.name = $2
            AND game.name = $3
        ), CURRENT_TIMESTAMP)",
        match_result as MatchResult,
        character,
        game,
    )
    .execute(exec)
    .await?;
    Ok(())
}
