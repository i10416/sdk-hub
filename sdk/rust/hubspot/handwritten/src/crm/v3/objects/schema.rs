use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProperties {
    #[serde(rename = "createDate", alias = "createdate")]
    pub create_date: Option<String>,
    pub hs_object_id: String,
    pub hs_lastmodifieddate: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct GetObjectRequest {
    // path
    pub name: String,
    // path
    pub id: String,
    pub fields: Vec<&'static str>,
}

impl GetObjectRequest {
    pub fn new(name: &str, id: &str, fields: &[&'static str]) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            fields: fields.to_vec(),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct DeleteObjectRequest {
    // path
    pub name: String,
    // path
    pub id: String,
}

impl DeleteObjectRequest {
    pub fn new(name: &str, id: &str) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateObjectRequest<P> {
    // path
    #[serde(skip)]
    pub name: String,
    pub properties: P,
}

#[derive(Debug, Serialize)]
pub struct UpdateObjectRequest<T: Serialize> {
    // path
    #[serde(skip)]
    pub name: String,
    // path
    #[serde(skip)]
    pub id: String,
    // body
    pub properties: T,
}
impl<T: Serialize> UpdateObjectRequest<T> {
    pub fn new(name: &str, id: &str, properties: T) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            properties,
        }
    }
}
