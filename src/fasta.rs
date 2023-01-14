#![allow(clippy::module_inception)]

mod error;
mod fasta;
mod fasta_metadata;
mod fasta_sequence;
mod parser;

pub(crate) use self::error::{Error, Result};
pub(crate) use self::parser::Rule;

pub use self::fasta::Fasta;
pub use self::fasta_metadata::FastaMetadata;
pub use self::fasta_sequence::FastaSequence;
