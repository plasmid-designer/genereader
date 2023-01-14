#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FASTA error: {0}")]
    FastaError(#[from] crate::fasta::Error),
    #[error("FASTQ error: {0}")]
    FastqError(#[from] crate::fastq::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
