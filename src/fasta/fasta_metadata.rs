#[derive(Debug)]
pub struct FastaMetadata {
    sequence_header: String,
}

impl FastaMetadata {
    pub(crate) fn new(sequence_header: String) -> Self {
        Self { sequence_header }
    }

    pub fn sequence_name(&self) -> &str {
        &self.sequence_header
    }
}
