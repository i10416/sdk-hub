use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use std::io::ErrorKind;

use crate::crm::prelude::{List, Object, Pagination};

pub mod companies;
pub mod deals;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProperties {
    #[serde(rename = "createDate", alias = "createdate")]
    pub create_date: Option<String>,
    pub hs_object_id: String,
    pub hs_lastmodifieddate: Option<String>,
}

impl crate::HubAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn v3_create_object<P: Debug + Serialize, R: DeserializeOwned>(
        &self,
        req: CreateObjectRequest<P>,
    ) -> Result<Object<R>, crate::crm::v3::Error> {
        let response = self
            .client
            .post(format!(
                "https://api.hubapi.com/crm/v3/objects/{name}",
                name = req.name,
            ))
            .json(&req)
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

    pub async fn v3_list_objects<T: DeserializeOwned>(
        &self,
        name: &str,
        pagination: Pagination,
    ) -> Result<List<T>, crate::crm::v3::Error> {
        let response = self
            .client
            .get(format!("https://api.hubapi.com/crm/v3/objects/{name}",))
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
    pub async fn v3_get_object<T: DeserializeOwned>(
        &self,
        GetObjectRequest { name, id, fields }: GetObjectRequest,
    ) -> Result<Object<T>, crate::crm::v3::Error> {
        let response = self
            .client
            .get(format!("https://api.hubapi.com/crm/v3/objects/{name}/{id}",))
            .query(&[("properties", fields.join(","))])
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
    pub async fn v3_update_object<T: Debug + Serialize + DeserializeOwned>(
        &self,
        req: UpdateObjectRequest<T>,
    ) -> Result<Object<T>, crate::crm::v3::Error> {
        let response = self
            .client
            .patch(format!(
                "https://api.hubapi.com/crm/v3/objects/{name}/{id}",
                name = req.name,
                id = req.id
            ))
            .json(&req)
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
    pub async fn v3_delete_object(
        &self,
        DeleteObjectRequest { name, id }: DeleteObjectRequest,
    ) -> Result<(), crate::crm::v3::Error> {
        let response = self
            .client
            .delete(format!("https://api.hubapi.com/crm/v3/objects/{name}/{id}",))
            .send()
            .await?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
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
pub struct GetObjectRequest {
    // path
    name: String,
    // path
    id: String,
    fields: Vec<&'static str>,
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
    name: String,
    // path
    id: String,
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
    name: String,
    properties: P,
}

#[derive(Debug, Serialize)]
pub struct UpdateObjectRequest<T: Serialize> {
    // path
    #[serde(skip)]
    name: String,
    // path
    #[serde(skip)]
    id: String,
    // body
    properties: T,
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
