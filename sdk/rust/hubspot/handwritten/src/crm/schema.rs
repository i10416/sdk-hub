use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropertyValue {
    #[serde(default)]
    versions: Vec<PropertyValueVersion>,
    #[serde(default)]
    value: String,
    source: String,
    timestamp: u64,
    source_id: String,
    request_id: String,
    #[serde(default)]
    source_upstream_deployable: Option<String>,
    #[serde(default)]
    is_encrypted: bool,
    sensitivity_level: String,
    updated_by_user_id: i32,
    persistence_timestamp: u64,
}
impl PropertyValue {
    pub fn new(
        versions: &[PropertyValueVersion],
        value: &str,
        source: &str,
        timestamp: u64,
        source_id: &str,
        request_id: &str,
        source_upstream_deployable: Option<&str>,
        is_encrypted: bool,
        sensitivity_level: &str,
        updated_by_user_id: i32,
        persistence_timestamp: u64,
    ) -> Self {
        Self {
            versions: versions.to_vec(),
            value: value.to_string(),
            source: source.to_string(),
            timestamp,
            source_id: source_id.to_string(),
            request_id: request_id.to_string(),
            source_upstream_deployable: source_upstream_deployable.map(str::to_string),
            is_encrypted,
            sensitivity_level: sensitivity_level.to_string(),
            updated_by_user_id,
            persistence_timestamp,
        }
    }
    pub fn versions(&self) -> &Vec<PropertyValueVersion> {
        &self.versions
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn persistence_timestamp(&self) -> u64 {
        self.persistence_timestamp
    }
    pub fn is_encrypted(&self) -> bool {
        self.is_encrypted
    }
    pub fn source_id(&self) -> &str {
        &self.source_id
    }
    pub fn updated_by_user_id(&self) -> i32 {
        self.updated_by_user_id
    }

    pub fn sensitivity_level(&self) -> &str {
        &self.sensitivity_level
    }
    pub fn request_id(&self) -> &str {
        &self.request_id
    }
    pub fn source_upstream_deployable(&self) -> Option<&str> {
        self.source_upstream_deployable.as_deref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropertyValueVersion {
    name: Option<String>,
    #[serde(default)]
    value: String,
    source: String,
    timestamp: u64,
    source_id: String,
    request_id: String,
    #[serde(default)]
    source_upstream_deployable: Option<String>,
    #[serde(default)]
    is_encrypted: bool,
    updated_by_user_id: i32,
    #[serde(default)]
    use_timestamp_as_persistence_timestamp: bool,
}

impl PropertyValueVersion {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn use_timestamp_as_persistence_timestamp(&self) -> bool {
        self.use_timestamp_as_persistence_timestamp
    }
    pub fn is_encrypted(&self) -> bool {
        self.is_encrypted
    }
    pub fn source_id(&self) -> &str {
        &self.source_id
    }
    pub fn updated_by_user_id(&self) -> i32 {
        self.updated_by_user_id
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }
    pub fn source_upstream_deployable(&self) -> Option<&str> {
        self.source_upstream_deployable.as_deref()
    }
}
