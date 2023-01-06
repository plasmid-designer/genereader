use crate::fasta::parser::FastaFileParser;

use super::FastaSection;

/// A multisequence FASTA file.
#[derive(Debug)]
pub struct FastaFile {
    pub(crate) sections: Vec<FastaSection>,
}

impl ToString for FastaFile {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for section in &self.sections {
            s.push_str(section.to_string().as_str());
            s.push_str("\n\n");
        }
        s.trim().to_string()
    }
}

impl FastaFile {
    pub fn parse(s: &str) -> Self {
        FastaFileParser::new().parse(s).unwrap()
    }

    pub fn sections(&self) -> &[FastaSection] {
        &self.sections
    }
}