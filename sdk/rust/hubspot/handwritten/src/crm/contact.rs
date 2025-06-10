use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactPropertyValue {
    #[serde(default)]
    value: String,
    #[serde(default)]
    versions: Vec<ContactPropertyValueVersion>,
}

impl ContactPropertyValue {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn versions(&self) -> &Vec<ContactPropertyValueVersion> {
        &self.versions
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ContactPropertyValueVersion {
    #[serde(default)]
    value: String,
    data_sensitivity: Option<String>,
    #[serde(default)]
    is_encrypted: Option<bool>,
    #[serde(default)]
    selected: Option<bool>,
    source_id: String,
    source_label: Option<String>,
    source_type: String,
    timestamp: u64,
    updated_by_user_id: Option<u64>,
}

impl ContactPropertyValueVersion {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn data_sensitivity(&self) -> Option<&str> {
        self.data_sensitivity.as_deref()
    }

    pub fn is_encrypted(&self) -> Option<bool> {
        self.is_encrypted
    }

    pub fn selected(&self) -> Option<bool> {
        self.selected
    }

    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn source_label(&self) -> Option<&str> {
        self.source_label.as_deref()
    }

    pub fn source_type(&self) -> &str {
        &self.source_type
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn updated_by_user_id(&self) -> Option<u64> {
        self.updated_by_user_id
    }
}
