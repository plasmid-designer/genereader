use pest::{iterators::Pair, Parser};

use crate::{helper::*, FileFormat};

use super::{
    parser::{FastqParser, Rule},
    FastqMetadata, FastqSequence,
};

#[derive(Debug)]
pub struct Fastq {
    sequences: Vec<FastqSequence>,
}

impl FileFormat for Fastq {
    const NAME: &'static str = "FASTQ";
    const EXTENSIONS: &'static [&'static str] = &["fq", "fastq"];
}

impl Fastq {
    pub fn sequences_iter(&self) -> impl Iterator<Item = &FastqSequence> {
        self.sequences.iter()
    }

    pub fn sequences_into_iter(self) -> impl Iterator<Item = FastqSequence> {
        self.sequences.into_iter()
    }
}

impl Fastq {
    pub fn parse(source: &str) -> crate::Result<Self> {
        let root = FastqParser::parse(Rule::fastq, source)
            .map_err(|err| Box::new(super::Error::FastaParseError(err)))?
            .next()
            .ok_or_else(|| {
                Box::new(super::Error::FastqCompileError {
                    expected: Rule::fastq,
                    actual: None,
                })
            })?;
        Ok(Self {
            sequences: Self::parse_root(root)?,
        })
    }

    fn parse_root(root: Pair<Rule>) -> super::Result<Vec<FastqSequence>> {
        let mut sequences = Vec::new();
        let root = root.expect(Rule::fastq)?;
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

    fn parse_definition(singleseq_def: Pair<Rule>) -> super::Result<FastqSequence> {
        let pair = singleseq_def.expect(Rule::singleseq_def)?;
        let mut pairs = pair.into_inner();

        let metadata = Self::parse_definition_metadata(pairs.next())?;
        let sequence = Self::parse_definition_sequence(pairs.next())?;
        let _quality_header = pairs.next().expect_some(Rule::quality_header)?;
        let quality = Self::parse_definition_quality(pairs.next())?;

        Ok(FastqSequence::new(metadata, sequence, quality))
    }

    fn parse_definition_metadata(
        sequence_header: Option<Pair<Rule>>,
    ) -> super::Result<FastqMetadata> {
        let pair = sequence_header.expect_some(Rule::sequence_header)?;
        let sequence_header = pair.as_str().trim_start_matches('@').to_string();
        Ok(FastqMetadata::new(sequence_header))
    }

    fn parse_definition_sequence(sequence_multiline: Option<Pair<Rule>>) -> super::Result<String> {
        let pair = sequence_multiline.expect_some(Rule::sequence_multiline)?;
        let sequence = pair.as_str().replace(['\r', '\n'], "");
        Ok(sequence)
    }

    fn parse_definition_quality(quality_multiline: Option<Pair<Rule>>) -> super::Result<String> {
        let pair = quality_multiline.expect_some(Rule::quality_multiline)?;
        let quality = pair.as_str().replace(['\r', '\n'], "");
        Ok(quality)
    }
}

#[cfg(test)]
mod tests {
    use super::Fastq;
    use indoc::indoc;

    #[test]
    fn test_fastq_parse() {
        let source = indoc! {"
            @Rosalind_6404
            CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG
            +Rosalind_6404
            !''*((((***+))%%%++)(%%%%).1***-+*''))**55CCF>>>>>>CCCCCCC65IIIIIIIIIIIIIIIIIIII
        "};
        let fastq = Fastq::parse(source).unwrap();
        assert_eq!(fastq.sequences.len(), 1);
        assert_eq!(fastq.sequences[0].sequence_name(), "Rosalind_6404");
        assert_eq!(
            fastq.sequences[0].sequence_str(),
            "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG"
        );
        assert_eq!(
            fastq.sequences[0].quality_str(),
            "!''*((((***+))%%%++)(%%%%).1***-+*''))**55CCF>>>>>>CCCCCCC65IIIIIIIIIIIIIIIIIIII"
        );

        let source = indoc! {"
            @Rosalind_6404
            CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG
            +Rosalind_6404
            !''*((((***+))%%%++)(%%%%).1***-+*''))**55CCF>>>>>>CCCCCCC65IIIIIIIIIIIIIIIIIIII
            @Rosalind_5959
            CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGAC
            +Rosalind_5959
            @'''))***(AAAAAAABBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
            @Rosalind_0808
            CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAG
            +Rosalind_0808
            +IIIIIIIIIIIII****((((((***+))%%%++)(%%%%).1***-+*''))**55CCF>>>>>>CCCCCCC65IIII

        "};
        let fastq = Fastq::parse(source).unwrap();
        assert_eq!(fastq.sequences.len(), 3);
        assert_eq!(fastq.sequences[0].sequence_name(), "Rosalind_6404");
        assert_eq!(
            fastq.sequences[0].sequence_str(),
            "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG"
        );
        assert_eq!(
            fastq.sequences[0].quality_str(),
            "!''*((((***+))%%%++)(%%%%).1***-+*''))**55CCF>>>>>>CCCCCCC65IIIIIIIIIIIIIIIIIIII"
        );
        assert_eq!(fastq.sequences[1].sequence_name(), "Rosalind_5959");
        assert_eq!(
            fastq.sequences[1].sequence_str(),
            "CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGAC"
        );
        assert_eq!(
            fastq.sequences[1].quality_str(),
            "@'''))***(AAAAAAABBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC"
        );
        assert_eq!(fastq.sequences[2].sequence_name(), "Rosalind_0808");
        assert_eq!(
            fastq.sequences[2].sequence_str(),
            "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAG"
        );
        assert_eq!(
            fastq.sequences[2].quality_str(),
            "+IIIIIIIIIIIII****((((((***+))%%%++)(%%%%).1***-+*''))**55CCF>>>>>>CCCCCCC65IIII"
        );
    }
}
