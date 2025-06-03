use serde::Deserialize;

use crate::crm::v3::objects::ObjectProperties;

#[derive(Debug, Deserialize)]
pub struct Company {
    #[serde(flatten)]
    pub shared_properties: ObjectProperties,
    pub domain: Option<String>,
    pub name: String,
}
