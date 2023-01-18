use indexmap::IndexMap;

#[derive(Debug)]
pub struct GenbankFeatureQualifier {
    key: String,
    value: String,
}

impl GenbankFeatureQualifier {
    pub(crate) fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug)]
pub struct GenbankFeature {
    key: String,
    location: String,
    qualifiers: IndexMap<String, GenbankFeatureQualifier>,
}

impl GenbankFeature {
    pub(crate) fn new(
        key: String,
        location: String,
        qualifiers: Vec<GenbankFeatureQualifier>,
    ) -> Self {
        Self {
            key,
            location,
            qualifiers: qualifiers
                .into_iter()
                .map(|q| (q.key().to_string(), q))
                .collect(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn qualifiers(&self) -> impl Iterator<Item = &GenbankFeatureQualifier> {
        self.qualifiers.values()
    }

    pub fn get_qualifier(&self, key: &str) -> Option<&str> {
        self.qualifiers.get(key).map(GenbankFeatureQualifier::value)
    }
}

#[derive(Debug, Default)]
pub struct GenbankFeatureTable {
    features: Vec<GenbankFeature>,
}

impl GenbankFeatureTable {
    pub(crate) fn new(features: Vec<GenbankFeature>) -> Self {
        Self { features }
    }

    pub fn features_iter(&self) -> impl Iterator<Item = &GenbankFeature> {
        self.features.iter()
    }
}
