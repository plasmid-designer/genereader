use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "grammar/genbank.pest"]
pub struct GenbankParser;

impl crate::RuleExt for Rule {
    type ERROR = super::Error;

    fn to_error(expected: Option<Self>, actual: Option<Self>) -> Self::ERROR {
        super::Error::GenbankCompileError { expected, actual }
    }
}
