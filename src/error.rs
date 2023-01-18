#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "fasta")]
    #[error("FASTA error: {0}")]
    FastaError(#[from] Box<crate::fasta::Error>),
    #[cfg(feature = "fastq")]
    #[error("FASTQ error: {0}")]
    FastqError(#[from] Box<crate::fastq::Error>),
    #[cfg(feature = "genbank")]
    #[error("Genbank error: {0}")]
    GenbankError(#[from] Box<crate::genbank::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
