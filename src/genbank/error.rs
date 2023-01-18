type GenbankParseError = pest::error::Error<super::Rule>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Genbank parsing error: {0}")]
    GenbankParseError(#[from] GenbankParseError),
    #[error("Genbank compilation error: Expected {expected:?}; Actual: {actual:?}")]
    GenbankCompileError {
        expected: Option<super::Rule>,
        actual: Option<super::Rule>,
    },
}

pub type Result<T> = std::result::Result<T, Box<Error>>;
