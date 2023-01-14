#[derive(Debug)]
pub struct FastqMetadata {
    sequence_header: String,
}

impl FastqMetadata {
    pub(crate) fn new(sequence_header: String) -> Self {
        Self { sequence_header }
    }

    pub fn sequence_name(&self) -> &str {
        &self.sequence_header
    }
}
