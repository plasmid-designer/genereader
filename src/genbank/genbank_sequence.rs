use super::{GenbankFeatureTable, GenbankMetadataTable};

#[derive(Debug)]
pub struct GenbankSequence {
    metadata: GenbankMetadataTable,
    features: GenbankFeatureTable,
    sequence: String,
}

impl GenbankSequence {
    pub(crate) fn new(
        metadata: GenbankMetadataTable,
        features: GenbankFeatureTable,
        sequence: String,
    ) -> Self {
        Self {
            metadata,
            features,
            sequence,
        }
    }

    pub fn metadata(&self) -> &GenbankMetadataTable {
        &self.metadata
    }

    pub fn features(&self) -> &GenbankFeatureTable {
        &self.features
    }

    pub fn sequence(&self) -> &str {
        &self.sequence
    }
}
