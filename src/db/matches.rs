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
    executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
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
    .execute(executor)
    .await?;
    Ok(())
}

#[derive(Debug)]
pub struct MatchOverview {
    pub result: MatchResult,
    pub count: i64,
}

pub async fn get_match_overview<'c>(
    executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
    game: &str,
    character: &str,
) -> Result<Vec<MatchOverview>> {
    Ok(sqlx::query_as!(MatchOverview,
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
    .fetch_all(executor)
    .await?)

}

pub async fn delete_last<'c>(
    executor: impl sqlx::Executor<'c, Database = sqlx::Postgres>,
    game: &str,
    character: &str,
) -> Result<()> {
    sqlx::query!(
        "DELETE FROM match
        WHERE id = (
            SELECT max(match.id)
            FROM match
            JOIN character ON match.character_id = character.id
            JOIN game ON character.game_id = game.id
            WHERE game.name = $1
                AND character.name = $2
        )",
        game,
        character,
    )
    .execute(executor)
    .await?;
    Ok(())
}
