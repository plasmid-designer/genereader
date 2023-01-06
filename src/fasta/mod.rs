mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub fasta, "/fasta/fasta.rs");
