use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropertyValue {
    #[serde(default)]
    versions: Vec<PropertyValueVersion>,
    value: String,
    source: String,
    timestamp: u64,
    source_id: String,
    request_id: String,
    source_upstream_deployable: String,
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
        source_upstream_deployable: &str,
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
            source_upstream_deployable: source_upstream_deployable.to_string(),
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
    pub fn source_upstream_deployable(&self) -> &str {
        &self.source_upstream_deployable
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropertyValueVersion {
    name: String,
    value: String,
    source: String,
    timestamp: u64,
    source_id: String,
    request_id: String,
    source_upstream_deployable: String,
    is_encrypted: bool,
    sensitivity_level: String,
    updated_by_user_id: i32,
    persistence_timestamp: u64,
}

impl PropertyValueVersion {
    pub fn name(&self) -> &str {
        &self.name
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
    pub fn source_upstream_deployable(&self) -> &str {
        &self.source_upstream_deployable
    }
}
