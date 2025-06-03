use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(super) struct CreateLicenseeRequest {
    #[serde(rename = "aid")]
    pub app_id: String,
    pub manager_uids: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representatives: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateLicenseeRequest {
    pub licensee_id: String,
    pub manager_uids: String,
    pub name: String,
    pub representatives: Option<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct Representative {
    email: String,
}
impl Representative {
    pub fn new(email: &str) -> Representative {
        Self {
            email: email.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]

pub(super) struct LicenseeResult {
    #[serde(alias = "Licensee")]
    pub licensee: Licensee,
}

#[derive(Debug, Serialize, Default)]
pub struct ListLicenseeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListLicenseeResult {
    pub licensees: Vec<Licensee>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Licensee {
    licensee_id: String,
    name: String,
}

impl Licensee {
    pub fn licensee_id(&self) -> &str {
        &self.licensee_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
