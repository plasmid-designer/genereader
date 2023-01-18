use pest::{iterators::Pair, Parser};

use crate::{helper::*, FileFormat};

use super::{
    parser::{FastaParser, Rule},
    FastaMetadata, FastaSequence,
};

#[derive(Debug)]
pub struct Fasta {
    sequences: Vec<FastaSequence>,
}

impl FileFormat for Fasta {
    const NAME: &'static str = "FASTA";
    const EXTENSIONS: &'static [&'static str] = &["fa", "faa", "fas", "fasta", "ffn", "fna", "frn"];
}

impl Fasta {
    pub fn parse(source: &str) -> crate::Result<Self> {
        let root = FastaParser::parse(Rule::fasta, source)
            .map_err(|err| Box::new(super::Error::FastaParseError(err)))?
            .next()
            .ok_or_else(|| {
                Box::new(super::Error::FastaCompileError {
                    expected: Some(Rule::fasta),
                    actual: None,
                })
            })?;
        Ok(Self {
            sequences: Self::parse_root(root)?,
        })
    }

    fn parse_root(root: Pair<Rule>) -> super::Result<Vec<FastaSequence>> {
        let mut sequences = Vec::new();
        let root = root.expect(Rule::fasta)?;
        for pair in root.into_inner() {
            match pair.as_rule() {
                Rule::multiseq_def => {
                    for pair in pair.into_inner() {
                        sequences.push(Self::parse_definition(pair)?)
                    }
                }
                Rule::EOI => break,
                _ => unreachable!(),
            }
        }
        Ok(sequences)
    }

    fn parse_definition(singleseq_def: Pair<Rule>) -> super::Result<FastaSequence> {
        let pair = singleseq_def.expect(Rule::singleseq_def)?;
        let mut pairs = pair.into_inner();

        let metadata = Self::parse_definition_metadata(pairs.next())?;
        let sequence = Self::parse_definition_sequence(pairs.next())?;

        Ok(FastaSequence::new(metadata, sequence))
    }

    fn parse_definition_metadata(
        sequence_header: Option<Pair<Rule>>,
    ) -> super::Result<FastaMetadata> {
        let pair = sequence_header.expect_some(Rule::sequence_header)?;
        let sequence_header = pair.as_str().trim_start_matches('>').to_string();
        Ok(FastaMetadata::new(sequence_header))
    }

    fn parse_definition_sequence(sequence_multiline: Option<Pair<Rule>>) -> super::Result<String> {
        let pair = sequence_multiline.expect_some(Rule::sequence_multiline)?;
        let sequence = pair.as_str().replace(['\r', '\n'], "");
        Ok(sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::Fasta;
    use indoc::indoc;

    #[test]
    fn test_fasta_parse() {
        let source = indoc! {"
            >Rosalind_6404
            CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCT
            CTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG
        "};
        let fasta = Fasta::parse(source).unwrap();
        assert_eq!(fasta.sequences.len(), 1);
        assert_eq!(fasta.sequences[0].sequence_name(), "Rosalind_6404");
        assert_eq!(
            fasta.sequences[0].sequence_str(),
            "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG"
        );

        let source = indoc! {"
            >Rosalind_6404
            CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCT
            CTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG
            >Rosalind_5959
            CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCC
            AGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGAC
            ACGC
            >Rosalind_0808
            CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAA
            CGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAG
            GTGGAAT
        "};
        let fasta = Fasta::parse(source).unwrap();
        assert_eq!(fasta.sequences.len(), 3);
        assert_eq!(fasta.sequences[0].sequence_name(), "Rosalind_6404");
        assert_eq!(
            fasta.sequences[0].sequence_str(),
            "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG"
        );
        assert_eq!(fasta.sequences[1].sequence_name(), "Rosalind_5959");
        assert_eq!(
            fasta.sequences[1].sequence_str(),
            "CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGACACGC"
        );
        assert_eq!(fasta.sequences[2].sequence_name(), "Rosalind_0808");
        assert_eq!(fasta.sequences[2].sequence_str(), "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT");
    }
}
