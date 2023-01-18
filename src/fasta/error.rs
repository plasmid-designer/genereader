type FastaParseError = pest::error::Error<super::Rule>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FASTA parsing error: {0}")]
    FastaParseError(#[from] FastaParseError),
    #[error("FASTA compilation error: Expected {expected:?}; Actual: {actual:?}")]
    FastaCompileError {
        expected: Option<super::Rule>,
        actual: Option<super::Rule>,
    },
}

pub type Result<T> = std::result::Result<T, Box<Error>>;
