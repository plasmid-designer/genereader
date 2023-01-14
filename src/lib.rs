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
mod fasta;

#[cfg(feature = "fasta")]
pub use fasta::Fasta;

//
// Public exports
// Feature: fastq
//

#[cfg(feature = "fastq")]
mod fastq;

#[cfg(feature = "fastq")]
pub use fastq::Fastq;
