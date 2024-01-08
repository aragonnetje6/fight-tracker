#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Data not found")]
    NotFoundError,
    #[error("Not a valid string")]
    ParseError,
}
