mod fasta_section;
mod fasta_file;

pub use fasta_section::FastaSection;
pub use fasta_file::FastaFile;

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
