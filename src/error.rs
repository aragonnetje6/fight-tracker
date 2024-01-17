#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not a valid string")]
    ParseError,
}
