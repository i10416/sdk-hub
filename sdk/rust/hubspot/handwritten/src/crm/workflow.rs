use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Object<T> {
    // Known Values: [DEAL, UNKNOWN]
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "objectId")]
    pub object_id: u64,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "objectTypeId")]
    pub object_type_id: String,
    pub properties: T,
    #[serde(default)]
    #[serde(rename = "is_deleted")]
    pub is_deleted: bool,
}
