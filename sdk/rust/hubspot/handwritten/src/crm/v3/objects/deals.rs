use crate::crm::{prelude::Object, v3::objects::GetObjectRequest};
use serde::Deserialize;

pub const BASE_ENDPOINT: &str = "https://api.hubapi.com/crm/v3/objects/deal";
type Resource = Deal;
const RESOURCE_NAME: &str = "deal";

impl crate::HubAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_deal(&self) -> Result<Object<Resource>, crate::crm::v3::Error> {
        todo!()
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_deal(&self, id: &str) -> Result<Object<Resource>, crate::crm::v3::Error> {
        let response = self
            .v3_get_object::<Resource>(GetObjectRequest {
                name: RESOURCE_NAME.to_string(),
                id: id.to_string(),
            })
            .await?;
        Ok(response)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_deal(&self) -> Result<Object<Resource>, crate::crm::v3::Error> {
        todo!()
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_deal(&self, id: &str) -> Result<(), crate::crm::v3::Error> {
        self.v3_delete_object(super::DeleteObjectRequest {
            name: RESOURCE_NAME.to_string(),
            id: id.to_string(),
        })
        .await
    }
}

#[derive(Debug, Deserialize)]
pub struct Deal {
    #[serde(flatten)]
    pub shared_properties: super::ObjectProperties,
    pub amount: Option<String>,
    #[serde(rename = "closeDate")]
    pub close_date: Option<String>,
    #[serde(rename = "createDate")]
    pub create_date: Option<String>,
    pub dealstage: Option<String>,
    pub dealname: String,
    pub pipeline: Option<String>,
}
