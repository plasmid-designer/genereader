#![allow(clippy::module_inception)]

mod error;
mod genbank;
mod genbank_feature_table;
mod genbank_metadata_table;
mod genbank_sequence;
mod parser;

pub(crate) use self::error::{Error, Result};
pub(crate) use self::parser::Rule;

pub use genbank::Genbank;

pub use self::genbank_feature_table::{GenbankFeature, GenbankFeatureTable};
pub use self::genbank_metadata_table::GenbankMetadataTable;
pub use self::genbank_sequence::GenbankSequence;
