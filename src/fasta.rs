mod ast;
pub use ast::{FastaSection, FastaFile};

lalrpop_util::lalrpop_mod!(pub parser, "/fasta/grammar/fasta.rs");