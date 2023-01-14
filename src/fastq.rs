#![allow(clippy::module_inception)]

mod error;
mod fastq;
mod fastq_metadata;
mod fastq_sequence;
mod parser;

pub(crate) use self::error::{Error, Result};
pub(crate) use self::parser::Rule;

pub use self::fastq::Fastq;
pub use self::fastq_metadata::FastqMetadata;
pub use self::fastq_sequence::FastqSequence;
