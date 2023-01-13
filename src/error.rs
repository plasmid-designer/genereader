#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FASTA error: {0}")]
    FastaError(#[from] crate::fasta::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
