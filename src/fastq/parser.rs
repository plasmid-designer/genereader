use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "grammar/fastq.pest"]
pub struct FastqParser;

impl crate::RuleExt for Rule {
    type ERROR = super::Error;

    fn to_error(expected: Self, actual: Option<Self>) -> Self::ERROR {
        super::Error::FastqCompileError { expected, actual }
    }
}
