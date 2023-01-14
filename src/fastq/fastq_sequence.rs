use super::FastqMetadata;

#[derive(Debug)]
pub struct FastqSequence {
    metadata: FastqMetadata,
    sequence: String,
    quality: String,
}

impl FastqSequence {
    pub(crate) fn new(metadata: FastqMetadata, sequence: String, quality: String) -> Self {
        Self {
            metadata,
            sequence,
            quality,
        }
    }

    pub fn sequence_name(&self) -> &str {
        &self.metadata.sequence_name()
    }

    pub fn sequence_str(&self) -> &str {
        &self.sequence
    }

    pub fn quality_str(&self) -> &str {
        &self.quality
    }
}
