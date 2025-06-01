use std::io::ErrorKind;

use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::crm::prelude::{List, Object, Pagination};

pub const BASE_ENDPOINT: &str = "https://api.hubapi.com/crm/v3/owners";

impl crate::HubAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn v3_list_owners<T: DeserializeOwned>(
        &self,
        pagination: Pagination,
    ) -> Result<List<T>, crate::crm::v3::Error> {
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn v3_get_owner<T: DeserializeOwned>(
        &self,
        GetOwnerRequest { id }: GetOwnerRequest,
    ) -> Result<Object<T>, crate::crm::v3::Error> {
        let response = self
            .client
            .get(format!("https://api.hubapi.com/crm/v3/owners/{id}",))
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
}

#[derive(Debug, Serialize)]
pub struct GetOwnerRequest {
    // path
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(default)]
    pub archived: bool,
    pub id: String,
    #[serde(rename = "type")]
    pub tpe: String,
    #[serde(rename = "userId")]
    pub user_id: usize,
    pub mail: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
