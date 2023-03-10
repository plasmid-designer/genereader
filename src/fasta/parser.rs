use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "grammar/fasta.pest"]
pub struct FastaParser;

impl crate::RuleExt for Rule {
    type ERROR = super::Error;

    fn to_error(expected: Option<Self>, actual: Option<Self>) -> Self::ERROR {
        super::Error::FastaCompileError { expected, actual }
    }
}
