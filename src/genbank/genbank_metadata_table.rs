use indexmap::IndexMap;

#[derive(Debug, Default)]
pub struct GenbankMetadataTable {
    map: IndexMap<String, String>,
}

impl GenbankMetadataTable {
    pub(crate) fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub(crate) fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }

    pub fn locus(&self) -> Option<&str> {
        self.get("LOCUS")
    }

    pub fn definition(&self) -> Option<&str> {
        self.get("DEFINITION")
    }
}
