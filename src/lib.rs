mod api;
mod error;
mod helper;

//
// Internal exports
//
pub(crate) use helper::RuleExt;

//
// Public exports
//

pub use api::*;
pub use error::{Error, Result};

//
// Public exports
// Feature: fasta
//

#[cfg(feature = "fasta")]
pub mod fasta;

#[cfg(feature = "fasta")]
pub use fasta::Fasta;

//
// Public exports
// Feature: fastq
//

#[cfg(feature = "fastq")]
pub mod fastq;

#[cfg(feature = "fastq")]
pub use fastq::Fastq;

//
// Public exports
// Feature: genbank
//

#[cfg(feature = "genbank")]
pub mod genbank;

#[cfg(feature = "genbank")]
pub use genbank::Genbank;
