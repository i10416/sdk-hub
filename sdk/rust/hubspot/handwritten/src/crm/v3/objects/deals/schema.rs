use serde::Deserialize;

use crate::crm::v3::objects::ObjectProperties;

#[derive(Debug, Deserialize)]
pub struct Deal {
    #[serde(flatten)]
    pub shared_properties: ObjectProperties,
    pub amount: Option<String>,
    #[serde(rename = "closeDate")]
    pub close_date: Option<String>,
    #[serde(rename = "createDate")]
    pub create_date: Option<String>,
    pub dealstage: Option<String>,
    pub dealname: String,
    pub pipeline: Option<String>,
}
