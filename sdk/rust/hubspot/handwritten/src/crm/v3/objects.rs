mod schema;
use reqwest::StatusCode;
pub use schema::*;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::io::ErrorKind;

use crate::crm::prelude::{Object, ObjectList, Pagination};

pub mod companies;
pub mod deals;

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
    ) -> Result<ObjectList<T>, crate::crm::v3::Error> {
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
