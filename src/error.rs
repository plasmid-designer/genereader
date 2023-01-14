#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FASTA error: {0}")]
    FastaError(#[from] Box<crate::fasta::Error>),
    #[error("FASTQ error: {0}")]
    FastqError(#[from] Box<crate::fastq::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
