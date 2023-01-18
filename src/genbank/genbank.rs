use pest::{iterators::Pair, Parser};

use crate::{
    helper::{PairExt, PairOptionExt},
    FileFormat,
};

use super::{
    genbank_feature_table::GenbankFeatureQualifier,
    parser::{GenbankParser, Rule},
    GenbankFeature, GenbankFeatureTable, GenbankMetadataTable, GenbankSequence,
};

#[derive(Debug)]
pub struct Genbank {
    pub sequence: GenbankSequence,
}

impl FileFormat for Genbank {
    const NAME: &'static str = "Genbank";
    const EXTENSIONS: &'static [&'static str] = &["gb", "gbk"];
}

impl Genbank {
    pub fn parse(source: &str) -> crate::Result<Self> {
        let root = GenbankParser::parse(Rule::root, source)
            .map_err(|err| Box::new(super::Error::GenbankParseError(err)))?
            .next()
            .ok_or_else(|| {
                Box::new(super::Error::GenbankCompileError {
                    expected: Some(Rule::root),
                    actual: None,
                })
            })?;
        Ok(Self {
            sequence: Self::parse_root(root)?,
        })
    }

    fn parse_root(root: Pair<Rule>) -> super::Result<GenbankSequence> {
        let mut root_iter = root.into_inner();

        let metadata_table = Self::parse_metadata_table(root_iter.next())?;
        let feature_table = Self::parse_feature_table(root_iter.next())?;
        let origin_sequence = Self::parse_origin_sequence(root_iter.next())?;
        root_iter.next().expect_some(Rule::EOI)?;

        Ok(GenbankSequence::new(
            metadata_table,
            feature_table,
            origin_sequence,
        ))
    }

    fn parse_metadata_table(
        metadata_table: Option<Pair<Rule>>,
    ) -> super::Result<GenbankMetadataTable> {
        let mut metadata_table_map = GenbankMetadataTable::default();
        let metadata_table = metadata_table.expect_some(Rule::metadata_table)?;

        for pair in metadata_table.into_inner() {
            let metadata_entry = pair.expect(Rule::metadata_entry)?;
            let mut metadata_entry_iter = metadata_entry.into_inner();

            let key = metadata_entry_iter
                .next()
                .expect_some(Rule::metadata_key)?
                .as_str();
            let value = metadata_entry_iter
                .next()
                .expect_some(Rule::metadata_value)?
                .as_str();

            metadata_entry_iter.next().expect_none()?;
            metadata_table_map.insert(key.into(), value.into());
        }

        Ok(metadata_table_map)
    }

    fn parse_feature_table(
        feature_table: Option<Pair<Rule>>,
    ) -> super::Result<GenbankFeatureTable> {
        let feature_table = feature_table.expect_some(Rule::feature_table)?;
        let mut features = Vec::new();

        for pair in feature_table.into_inner() {
            let feature_entry = pair.expect(Rule::feature_table_entry)?;
            let feature = Self::parse_feature_table_entry(feature_entry)?;
            features.push(feature);
        }

        Ok(GenbankFeatureTable::new(features))
    }

    fn parse_feature_table_entry(feature_table_entry: Pair<Rule>) -> super::Result<GenbankFeature> {
        let mut feature_table_entry_iter = feature_table_entry.into_inner();
        let mut key_value_iter = feature_table_entry_iter
            .next()
            .expect_some(Rule::feature_key_value)?
            .into_inner();
        let key = key_value_iter
            .next()
            .expect_some(Rule::feature_key)?
            .as_str();
        let location = key_value_iter
            .next()
            .expect_some(Rule::feature_value)?
            .as_str();
        let qualifiers =
            Self::parse_feature_table_entry_qualifiers(feature_table_entry_iter.next())?;
        let feature = GenbankFeature::new(key.into(), location.into(), qualifiers);
        feature_table_entry_iter.next().expect_none()?;

        Ok(feature)
    }

    fn parse_feature_table_entry_qualifiers(
        qualifier_list: Option<Pair<Rule>>,
    ) -> super::Result<Vec<GenbankFeatureQualifier>> {
        let mut qualifiers = Vec::new();
        let qualifier_list = qualifier_list.expect_some(Rule::qualifier_list)?;

        for pair in qualifier_list.into_inner() {
            let qualifier_entry = pair.expect(Rule::qualifier_entry)?;
            let mut qualifier_iter = qualifier_entry.into_inner();
            let qualifier_key = qualifier_iter
                .next()
                .expect_some(Rule::qualifier_key)?
                .as_str()
                .trim_start_matches('/');
            let qualifier_value = qualifier_iter
                .next()
                .expect_some(Rule::qualifier_value)?
                .as_str()
                .trim_matches('"');

            qualifiers.push(GenbankFeatureQualifier::new(
                qualifier_key.into(),
                qualifier_value.into(),
            ));
        }

        Ok(qualifiers)
    }

    fn parse_origin_sequence(origin_sequence: Option<Pair<Rule>>) -> super::Result<String> {
        let origin_sequence = origin_sequence.expect_some(Rule::origin_block)?;
        let mut final_sequence = String::new();

        for pair in origin_sequence.into_inner() {
            let origin_line = pair.expect(Rule::origin_line)?;
            let mut origin_line_iter = origin_line.into_inner();
            let _ = origin_line_iter
                .next()
                .expect_some(Rule::origin_line_number)?;
            let origin_line_sequence = origin_line_iter
                .next()
                .expect_some(Rule::origin_line_sequence)?
                .as_str()
                .replace(char::is_whitespace, "");
            final_sequence.push_str(&origin_line_sequence);
        }

        Ok(final_sequence)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_genbank_parse() {
        // Sample record taken from https://www.ncbi.nlm.nih.gov/Sitemap/samplerecord.html
        let input = indoc! {
            r##"
            LOCUS       SCU49845     5028 bp    DNA             PLN       21-JUN-1999
            DEFINITION  Saccharomyces cerevisiae TCP1-beta gene, partial cds, and Axl2p
                        (AXL2) and Rev7p (REV7) genes, complete cds.
            ACCESSION   U49845
            VERSION     U49845.1  GI:1293613
            KEYWORDS    .
            SOURCE      Saccharomyces cerevisiae (baker's yeast)
            ORGANISM  Saccharomyces cerevisiae
                        Eukaryota; Fungi; Ascomycota; Saccharomycotina; Saccharomycetes;
                        Saccharomycetales; Saccharomycetaceae; Saccharomyces.
            REFERENCE   1  (bases 1 to 5028)
            AUTHORS   Torpey,L.E., Gibbs,P.E., Nelson,J. and Lawrence,C.W.
            TITLE     Cloning and sequence of REV7, a gene whose function is required for
                        DNA damage-induced mutagenesis in Saccharomyces cerevisiae
            JOURNAL   Yeast 10 (11), 1503-1509 (1994)
            PUBMED    7871890
            REFERENCE   2  (bases 1 to 5028)
            AUTHORS   Roemer,T., Madden,K., Chang,J. and Snyder,M.
            TITLE     Selection of axial growth sites in yeast requires Axl2p, a novel
                        plasma membrane glycoprotein
            JOURNAL   Genes Dev. 10 (7), 777-793 (1996)
            PUBMED    8846915
            REFERENCE   3  (bases 1 to 5028)
            AUTHORS   Roemer,T.
            TITLE     Direct Submission
            JOURNAL   Submitted (22-FEB-1996) Terry Roemer, Biology, Yale University, New
                        Haven, CT, USA
            FEATURES             Location/Qualifiers
                source          1..5028
                                /organism="Saccharomyces cerevisiae"
                                /db_xref="taxon:4932"
                                /chromosome="IX"
                                /map="9"
                CDS             <1..206
                                /codon_start=3
                                /product="TCP1-beta"
                                /protein_id="AAA98665.1"
                                /db_xref="GI:1293614"
                                /translation="SSIYNGISTSGLDLNNGTIADMRQLGIVESYKLKRAVVSSASEA
                                AEVLLRVDNIIRARPRTANRQHM"
                gene            687..3158
                                /gene="AXL2"
                CDS             687..3158
                                /gene="AXL2"
                                /note="plasma membrane glycoprotein"
                                /codon_start=1
                                /function="required for axial budding pattern of S.
                                cerevisiae"
                                /product="Axl2p"
                                /protein_id="AAA98666.1"
                                /db_xref="GI:1293615"
                                /translation="MTQLQISLLLTATISLLHLVVATPYEAYPIGKQYPPVARVNESF
                                TFQISNDTYKSSVDKTAQITYNCFDLPSWLSFDSSSRTFSGEPSSDLLSDANTTLYFN
                                VILEGTDSADSTSLNNTYQFVVTNRPSISLSSDFNLLALLKNYGYTNGKNALKLDPNE
                                VFNVTFDRSMFTNEESIVSYYGRSQLYNAPLPNWLFFDSGELKFTGTAPVINSAIAPE
                                TSYSFVIIATDIEGFSAVEVEFELVIGAHQLTTSIQNSLIINVTDTGNVSYDLPLNYV
                                YLDDDPISSDKLGSINLLDAPDWVALDNATISGSVPDELLGKNSNPANFSVSIYDTYG
                                DVIYFNFEVVSTTDLFAISSLPNINATRGEWFSYYFLPSQFTDYVNTNVSLEFTNSSQ
                                DHDWVKFQSSNLTLAGEVPKNFDKLSLGLKANQGSQSQELYFNIIGMDSKITHSNHSA
                                NATSTRSSHHSTSTSSYTSSTYTAKISSTSAAATSSAPAALPAANKTSSHNKKAVAIA
                                CGVAIPLGVILVALICFLIFWRRRRENPDDENLPHAISGPDLNNPANKPNQENATPLN
                                NPFDDDASSYDDTSIARRLAALNTLKLDNHSATESDISSVDEKRDSLSGMNTYNDQFQ
                                SQSKEELLAKPPVQPPESPFFDPQNRSSSVYMDSEPAVNKSWRYTGNLSPVSDIVRDS
                                YGSQKTVDTEKLFDLEAPEKEKRTSRDVTMSSLDPWNSNISPSPVRKSVTPSPYNVTK
                                HRNRHLQNIQDSQSGKNGITPTTMSTSSSDDFVPVKDGENFCWVHSMEPDRRPSKKRL
                                VDFSNKSNVNVGQVKDIHGRIPEML"
                gene            complement(3300..4037)
                                /gene="REV7"
                CDS             complement(3300..4037)
                                /gene="REV7"
                                /codon_start=1
                                /product="Rev7p"
                                /protein_id="AAA98667.1"
                                /db_xref="GI:1293616"
                                /translation="MNRWVEKWLRVYLKCYINLILFYRNVYPPQSFDYTTYQSFNLPQ
                                FVPINRHPALIDYIEELILDVLSKLTHVYRFSICIINKKNDLCIEKYVLDFSELQHVD
                                KDDQIITETEVFDEFRSSLNSLIMHLEKLPKVNDDTITFEAVINAIELELGHKLDRNR
                                RVDSLEEKAEIERDSNWVKCQEDENLPDNNGFQPPKIKLTSLVGSDVGPLIIHQFSEK
                                LISGDDKILNGVYSQYEEGESIFGSLF"
            ORIGIN
                    1 gatcctccat atacaacggt atctccacct caggtttaga tctcaacaac ggaaccattg
                61 ccgacatgag acagttaggt atcgtcgaga gttacaagct aaaacgagca gtagtcagct
                121 ctgcatctga agccgctgaa gttctactaa gggtggataa catcatccgt gcaagaccaa
                181 gaaccgccaa tagacaacat atgtaacata tttaggatat acctcgaaaa taataaaccg
                241 ccacactgtc attattataa ttagaaacag aacgcaaaaa ttatccacta tataattcaa
                301 agacgcgaaa aaaaaagaac aacgcgtcat agaacttttg gcaattcgcg tcacaaataa
                361 attttggcaa cttatgtttc ctcttcgagc agtactcgag ccctgtctca agaatgtaat
                421 aatacccatc gtaggtatgg ttaaagatag catctccaca acctcaaagc tccttgccga
                481 gagtcgccct cctttgtcga gtaattttca cttttcatat gagaacttat tttcttattc
                541 tttactctca catcctgtag tgattgacac tgcaacagcc accatcacta gaagaacaga
                601 acaattactt aatagaaaaa ttatatcttc ctcgaaacga tttcctgctt ccaacatcta
                661 cgtatatcaa gaagcattca cttaccatga cacagcttca gatttcatta ttgctgacag
                721 ctactatatc actactccat ctagtagtgg ccacgcccta tgaggcatat cctatcggaa
                781 aacaataccc cccagtggca agagtcaatg aatcgtttac atttcaaatt tccaatgata
                841 cctataaatc gtctgtagac aagacagctc aaataacata caattgcttc gacttaccga
                901 gctggctttc gtttgactct agttctagaa cgttctcagg tgaaccttct tctgacttac
                961 tatctgatgc gaacaccacg ttgtatttca atgtaatact cgagggtacg gactctgccg
                1021 acagcacgtc tttgaacaat acataccaat ttgttgttac aaaccgtcca tccatctcgc
                1081 tatcgtcaga tttcaatcta ttggcgttgt taaaaaacta tggttatact aacggcaaaa
                1141 acgctctgaa actagatcct aatgaagtct tcaacgtgac ttttgaccgt tcaatgttca
                1201 ctaacgaaga atccattgtg tcgtattacg gacgttctca gttgtataat gcgccgttac
                1261 ccaattggct gttcttcgat tctggcgagt tgaagtttac tgggacggca ccggtgataa
                1321 actcggcgat tgctccagaa acaagctaca gttttgtcat catcgctaca gacattgaag
                1381 gattttctgc cgttgaggta gaattcgaat tagtcatcgg ggctcaccag ttaactacct
                1441 ctattcaaaa tagtttgata atcaacgtta ctgacacagg taacgtttca tatgacttac
                1501 ctctaaacta tgtttatctc gatgacgatc ctatttcttc tgataaattg ggttctataa
                1561 acttattgga tgctccagac tgggtggcat tagataatgc taccatttcc gggtctgtcc
                1621 cagatgaatt actcggtaag aactccaatc ctgccaattt ttctgtgtcc atttatgata
                1681 cttatggtga tgtgatttat ttcaacttcg aagttgtctc cacaacggat ttgtttgcca
                1741 ttagttctct tcccaatatt aacgctacaa ggggtgaatg gttctcctac tattttttgc
                1801 cttctcagtt tacagactac gtgaatacaa acgtttcatt agagtttact aattcaagcc
                1861 aagaccatga ctgggtgaaa ttccaatcat ctaatttaac attagctgga gaagtgccca
                1921 agaatttcga caagctttca ttaggtttga aagcgaacca aggttcacaa tctcaagagc
                1981 tatattttaa catcattggc atggattcaa agataactca ctcaaaccac agtgcgaatg
                2041 caacgtccac aagaagttct caccactcca cctcaacaag ttcttacaca tcttctactt
                2101 acactgcaaa aatttcttct acctccgctg ctgctacttc ttctgctcca gcagcgctgc
                2161 cagcagccaa taaaacttca tctcacaata aaaaagcagt agcaattgcg tgcggtgttg
                2221 ctatcccatt aggcgttatc ctagtagctc tcatttgctt cctaatattc tggagacgca
                2281 gaagggaaaa tccagacgat gaaaacttac cgcatgctat tagtggacct gatttgaata
                2341 atcctgcaaa taaaccaaat caagaaaacg ctacaccttt gaacaacccc tttgatgatg
                2401 atgcttcctc gtacgatgat acttcaatag caagaagatt ggctgctttg aacactttga
                2461 aattggataa ccactctgcc actgaatctg atatttccag cgtggatgaa aagagagatt
                2521 ctctatcagg tatgaataca tacaatgatc agttccaatc ccaaagtaaa gaagaattat
                2581 tagcaaaacc cccagtacag cctccagaga gcccgttctt tgacccacag aataggtctt
                2641 cttctgtgta tatggatagt gaaccagcag taaataaatc ctggcgatat actggcaacc
                2701 tgtcaccagt ctctgatatt gtcagagaca gttacggatc acaaaaaact gttgatacag
                2761 aaaaactttt cgatttagaa gcaccagaga aggaaaaacg tacgtcaagg gatgtcacta
                2821 tgtcttcact ggacccttgg aacagcaata ttagcccttc tcccgtaaga aaatcagtaa
                2881 caccatcacc atataacgta acgaagcatc gtaaccgcca cttacaaaat attcaagact
                2941 ctcaaagcgg taaaaacgga atcactccca caacaatgtc aacttcatct tctgacgatt
                3001 ttgttccggt taaagatggt gaaaattttt gctgggtcca tagcatggaa ccagacagaa
                3061 gaccaagtaa gaaaaggtta gtagattttt caaataagag taatgtcaat gttggtcaag
                3121 ttaaggacat tcacggacgc atcccagaaa tgctgtgatt atacgcaacg atattttgct
                3181 taattttatt ttcctgtttt attttttatt agtggtttac agatacccta tattttattt
                3241 agtttttata cttagagaca tttaatttta attccattct tcaaatttca tttttgcact
                3301 taaaacaaag atccaaaaat gctctcgccc tcttcatatt gagaatacac tccattcaaa
                3361 attttgtcgt caccgctgat taatttttca ctaaactgat gaataatcaa aggccccacg
                3421 tcagaaccga ctaaagaagt gagttttatt ttaggaggtt gaaaaccatt attgtctggt
                3481 aaattttcat cttcttgaca tttaacccag tttgaatccc tttcaatttc tgctttttcc
                3541 tccaaactat cgaccctcct gtttctgtcc aacttatgtc ctagttccaa ttcgatcgca
                3601 ttaataactg cttcaaatgt tattgtgtca tcgttgactt taggtaattt ctccaaatgc
                3661 ataatcaaac tatttaagga agatcggaat tcgtcgaaca cttcagtttc cgtaatgatc
                3721 tgatcgtctt tatccacatg ttgtaattca ctaaaatcta aaacgtattt ttcaatgcat
                3781 aaatcgttct ttttattaat aatgcagatg gaaaatctgt aaacgtgcgt taatttagaa
                3841 agaacatcca gtataagttc ttctatatag tcaattaaag caggatgcct attaatggga
                3901 acgaactgcg gcaagttgaa tgactggtaa gtagtgtagt cgaatgactg aggtgggtat
                3961 acatttctat aaaataaaat caaattaatg tagcatttta agtataccct cagccacttc
                4021 tctacccatc tattcataaa gctgacgcaa cgattactat tttttttttc ttcttggatc
                4081 tcagtcgtcg caaaaacgta taccttcttt ttccgacctt ttttttagct ttctggaaaa
                4141 gtttatatta gttaaacagg gtctagtctt agtgtgaaag ctagtggttt cgattgactg
                4201 atattaagaa agtggaaatt aaattagtag tgtagacgta tatgcatatg tatttctcgc
                4261 ctgtttatgt ttctacgtac ttttgattta tagcaagggg aaaagaaata catactattt
                4321 tttggtaaag gtgaaagcat aatgtaaaag ctagaataaa atggacgaaa taaagagagg
                4381 cttagttcat cttttttcca aaaagcaccc aatgataata actaaaatga aaaggatttg
                4441 ccatctgtca gcaacatcag ttgtgtgagc aataataaaa tcatcacctc cgttgccttt
                4501 agcgcgtttg tcgtttgtat cttccgtaat tttagtctta tcaatgggaa tcataaattt
                4561 tccaatgaat tagcaatttc gtccaattct ttttgagctt cttcatattt gctttggaat
                4621 tcttcgcact tcttttccca ttcatctctt tcttcttcca aagcaacgat ccttctaccc
                4681 atttgctcag agttcaaatc ggcctctttc agtttatcca ttgcttcctt cagtttggct
                4741 tcactgtctt ctagctgttg ttctagatcc tggtttttct tggtgtagtt ctcattatta
                4801 gatctcaagt tattggagtc ttcagccaat tgctttgtat cagacaattg actctctaac
                4861 ttctccactt cactgtcgag ttgctcgttt ttagcggaca aagatttaat ctcgttttct
                4921 ttttcagtgt tagattgctc taattctttg agctgttctc tcagctcctc atatttttct
                4981 tgccatgact cagattctaa ttttaagcta ttcaatttct ctttgatc
            //
            "##
        };
        let genbank = Genbank::parse(input).unwrap();
        let metadata = genbank.sequence.metadata();
        assert_eq!(
            metadata.locus(),
            Some("SCU49845     5028 bp    DNA             PLN       21-JUN-1999")
        );
        assert_eq!(metadata.definition(), Some("Saccharomyces cerevisiae TCP1-beta gene, partial cds, and Axl2p\n            (AXL2) and Rev7p (REV7) genes, complete cds."));
        let features = genbank.sequence.features();
        let cds = features
            .features_iter()
            .find(|feature| feature.key() == "CDS")
            .unwrap();
        assert_eq!(cds.key(), "CDS");
        assert_eq!(cds.location(), "<1..206");
        assert_eq!(cds.get_qualifier("codon_start").unwrap(), "3");
        assert_eq!(cds.get_qualifier("product").unwrap(), "TCP1-beta");
        assert_eq!(cds.get_qualifier("translation").unwrap(), "SSIYNGISTSGLDLNNGTIADMRQLGIVESYKLKRAVVSSASEA\n                    AEVLLRVDNIIRARPRTANRQHM");
    }
}
