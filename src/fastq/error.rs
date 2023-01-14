type FastqParseError = pest::error::Error<super::Rule>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FASTQ parsing error: {0}")]
    FastaParseError(#[from] FastqParseError),
    #[error("FASTQ compilation error: Expected {expected:?}; Actual: {actual:?}")]
    FastqCompileError {
        expected: super::Rule,
        actual: Option<super::Rule>,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
