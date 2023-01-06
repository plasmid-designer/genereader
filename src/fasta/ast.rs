/// A single section of a multisequence FASTA file.
#[derive(Debug, Default, PartialEq)]
pub struct FastaSection {
    pub(crate) comments: Vec<String>,
    pub(crate) desc: String,
    pub(crate) seq: Vec<String>,
}

impl FastaSection {
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

/// A multisequence FASTA file.
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
        super::fasta::FastaFileParser::new().parse(s).unwrap()
    }

    pub fn sections(&self) -> &[FastaSection] {
        &self.sections
    }
}

#[cfg(test)]
mod tests {
    use super::FastaFile;

    #[test]
    fn test_fasta_file_parse() {
        let fasta = FastaFile::parse(
            "
;Test
>MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken
MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTI
DFPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIRE
ADIDGDGQVNYEEFVQMMTAK*

>MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken
MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTI
DFPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIRE
ADIDGDGQVNYEEFVQMMTAK*
"
            .trim(),
        );
        println!("{:#?}", fasta.sections[0].seq);
        assert_eq!(fasta.sections.len(), 2);
        assert_eq!(fasta.sections[0].comments, &[";Test"]);
        assert_eq!(
            fasta.sections[0].desc,
            ">MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken"
        );
        assert_eq!(fasta.sections[0].seq.len(), 3);
        assert_eq!(
            fasta.sections[0].seq[0],
            "MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTI"
        );
        assert_eq!(
            fasta.sections[0].seq[1],
            "DFPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIRE"
        );
        assert_eq!(fasta.sections[0].seq[2], "ADIDGDGQVNYEEFVQMMTAK*");
        assert!(fasta.sections[1].comments.is_empty());
        assert_eq!(
            fasta.sections[1].desc,
            ">MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken"
        );
        assert_eq!(fasta.sections[1].seq.len(), 3);
        assert_eq!(
            fasta.sections[1].seq[0],
            "MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTI"
        );
        assert_eq!(
            fasta.sections[1].seq[1],
            "DFPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIRE"
        );
        assert_eq!(fasta.sections[1].seq[2], "ADIDGDGQVNYEEFVQMMTAK*");
    }

    #[test]
    fn test_fasta_file_to_string() {
        let input = ("
;Test
>MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken
MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTI
DFPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIRE
ADIDGDGQVNYEEFVQMMTAK*

>MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken
MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTI
DFPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIRE
ADIDGDGQVNYEEFVQMMTAK*
")
        .trim();
        let fasta = FastaFile::parse(input);
        assert_eq!(fasta.to_string(), input);
    }
}
