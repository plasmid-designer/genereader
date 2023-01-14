mod api;
mod error;
mod fasta;
mod fastq;
mod helper;

pub(crate) use helper::RuleExt;

pub use api::*;
pub use error::{Error, Result};

pub use fasta::Fasta;
pub use fastq::Fastq;
