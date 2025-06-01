use serde::Deserialize;

use crate::crm::prelude::Object;

use super::{GetObjectRequest, ObjectProperties};

pub const BASE_ENDPOINT: &str = "https://api.hubapi.com/crm/v3/objects/companies";

type Resource = Company;
const RESOURCE_NAME: &str = "companies";

impl crate::HubAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_companies(&self) -> Result<Object<Resource>, crate::crm::v3::Error> {
        todo!()
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_companies(&self, id: &str) -> Result<Object<Resource>, crate::crm::v3::Error> {
        let response = self
            .v3_get_object::<Resource>(GetObjectRequest {
                name: RESOURCE_NAME.to_string(),
                id: id.to_string(),
            })
            .await?;
        Ok(response)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_companies(&self) -> Result<Object<Resource>, crate::crm::v3::Error> {
        todo!()
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_companies(&self, id: &str) -> Result<(), crate::crm::v3::Error> {
        self.v3_delete_object(super::DeleteObjectRequest {
            name: RESOURCE_NAME.to_string(),
            id: id.to_string(),
        })
        .await
    }
}

#[derive(Debug, Deserialize)]
pub struct Company {
    #[serde(flatten)]
    pub shared_properties: ObjectProperties,
    pub domain: Option<String>,
    pub name: String,
}
