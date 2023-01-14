use super::FastaMetadata;

#[derive(Debug)]
pub struct FastaSequence {
    metadata: FastaMetadata,
    sequence: String,
}

impl FastaSequence {
    pub(crate) fn new(metadata: FastaMetadata, sequence: String) -> Self {
        Self { metadata, sequence }
    }

    pub fn sequence_name(&self) -> &str {
        self.metadata.sequence_name()
    }

    pub fn sequence_str(&self) -> &str {
        &self.sequence
    }
}
