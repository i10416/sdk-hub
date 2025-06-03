mod schema;
pub use schema::*;

use std::io::ErrorKind;

use reqwest::StatusCode;

use crate::crm::prelude::{List, Pagination};

pub const BASE_ENDPOINT: &str = "https://api.hubapi.com/crm/v3/owners";

impl crate::HubAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn v3_list_owners(
        &self,
        pagination: Pagination,
    ) -> Result<List<Owner>, crate::crm::v3::Error> {
        let response = self
            .client
            .get(format!("https://api.hubapi.com/crm/v3/owners",))
            .query(&pagination)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => {
                let data = response.json().await?;
                Ok(data)
            }
            _ => {
                let value = response.json::<serde_json::Value>().await?;
                Err(Box::new(std::io::Error::new(
                    ErrorKind::Other,
                    value.to_string(),
                )))
            }
        }
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn v3_get_owner(
        &self,
        id: &str,
        params: GetOwnerRequest,
    ) -> Result<Option<Owner>, crate::crm::v3::Error> {
        let response = self
            .client
            .get(format!("https://api.hubapi.com/crm/v3/owners/{id}"))
            .query(&params)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => {
                let data = response.json().await?;
                Ok(data)
            }
            StatusCode::NOT_FOUND => Ok(None),
            _ => {
                let value = response.json::<serde_json::Value>().await?;
                Err(Box::new(std::io::Error::new(
                    ErrorKind::Other,
                    value.to_string(),
                )))
            }
        }
    }
}
