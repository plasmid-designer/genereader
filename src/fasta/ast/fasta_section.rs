/// A single section of a multisequence FASTA file.
#[derive(Debug, Default, PartialEq)]
pub struct FastaSection {
    pub(crate) comments: Vec<String>,
    pub(crate) desc: String,
    pub(crate) seq: Vec<String>,
}

impl FastaSection {
    fn description(&self) -> String {
        self.desc.clone()
    }

    /// Get the full nucleotide/aminoacid sequence.
    fn sequence(&self) -> String {
        self.seq.join("")
    }
}

impl ToString for FastaSection {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for comment in &self.comments {
            s.push_str(comment.as_str());
            s.push('\n');
        }
        s.push_str(self.desc.as_str());
        s.push('\n');
        for seq in &self.seq {
            s.push_str(seq.as_str());
            s.push('\n');
        }
        s.trim().to_string()
    }
}
