pub mod deals;

use std::fmt::Debug;
use std::io::ErrorKind;

use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// Resource type
#[derive(Debug, Deserialize)]
pub struct Object<T> {
    pub id: String,
    pub properties: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProperties {
    #[serde(rename = "createDate", alias = "createdate")]
    pub create_date: Option<String>,
    pub hs_object_id: String,
    pub hs_lastmodifieddate: Option<String>,
}

// List type
#[derive(Debug, Deserialize)]
pub struct List<T> {
    #[serde(default)]
    pub paging: Option<Paging>,
    pub results: Vec<Object<T>>,
}
#[derive(Debug, Deserialize)]
pub struct Paging {
    #[serde(default)]
    pub next: Option<Next>,
}

#[derive(Debug, Deserialize)]
pub struct Next {
    pub after: String,
    pub link: String,
}

#[derive(Debug, Serialize, Default)]
pub struct Pagination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl crate::HubAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn v4_create_nested_object<P: Debug + Serialize, R: DeserializeOwned>(
        &self,
        req: CreateNestedObjectRequest<P>,
    ) -> Result<Object<R>, crate::crm::v3::Error> {
        let response = self
            .client
            .post(format!(
                "https://api.hubapi.com/crm/v4/objects/{parent_name}/{parent_id}/{name}/{id}",
                parent_name = req.parent_name,
                parent_id = req.parent_id,
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

    pub async fn v4_list_nested_objects<T: DeserializeOwned>(
        &self,
        parent_name: &str,
        parent_id: &str,
        name: &str,
        pagination: Pagination,
    ) -> Result<List<T>, crate::crm::v3::Error> {
        let response = self
            .client
            .get(format!(
                "https://api.hubapi.com/crm/v4/objects/{parent_name}/{parent_id}/{name}",
            ))
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
    pub async fn v4_get_nested_object<T: DeserializeOwned>(
        &self,
        GetNestedObjectRequest {
            parent_name,
            parent_id,
            name,
            id,
        }: GetNestedObjectRequest,
    ) -> Result<Object<T>, crate::crm::v3::Error> {
        let response = self
            .client
            .get(format!(
                "https://api.hubapi.com/crm/v4/objects/{parent_name}/{parent_id}/{name}/{id}",
            ))
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
    pub async fn v4_update_nested_object<Input: Debug + Serialize, T: DeserializeOwned>(
        &self,
        req: UpdateObjectRequest<Input>,
    ) -> Result<Object<T>, crate::crm::v3::Error> {
        let response = self
            .client
            .patch(format!(
                "https://api.hubapi.com/crm/v4/objects/{parent_name}/{parent_id}/{name}/{id}",
                parent_name = req.parent_name,
                parent_id = req.parent_id,
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
    pub async fn v4_delete_nested_object(
        &self,
        DeleteNestedObjectRequest {
            parent_name,
            parent_id,
            name,
            id,
        }: DeleteNestedObjectRequest,
    ) -> Result<(), crate::crm::v3::Error> {
        let response = self
            .client
            .delete(format!(
                "https://api.hubapi.com/crm/v4/objects/{parent_name}/{parent_id}/{name}/{id}",
            ))
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
pub struct GetNestedObjectRequest {
    // path
    parent_name: String,
    // path
    parent_id: String,
    // path
    name: String,
    // path
    id: String,
}

impl GetNestedObjectRequest {
    pub fn new(parent_name: &str, parent_id: &str, name: &str, id: &str) -> Self {
        Self {
            parent_name: parent_name.to_string(),
            parent_id: parent_id.to_string(),
            name: name.to_string(),
            id: id.to_string(),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct DeleteNestedObjectRequest {
    // path
    parent_name: String,
    // path
    parent_id: String,
    // path
    name: String,
    // path
    id: String,
}

impl DeleteNestedObjectRequest {
    pub fn new(parent_name: &str, parent_id: &str, name: &str, id: &str) -> Self {
        Self {
            parent_name: parent_name.to_string(),
            parent_id: parent_id.to_string(),
            name: name.to_string(),
            id: id.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateNestedObjectRequest<P> {
    // path
    #[serde(skip)]
    pub parent_name: String,
    // path
    #[serde(skip)]
    pub parent_id: String,
    // path
    #[serde(skip)]
    pub name: String,
    // path
    #[serde(skip)]
    pub id: String,
    properties: P,
}

#[derive(Debug, Serialize)]
pub struct UpdateObjectRequest<T: Serialize> {
    // path
    #[serde(skip)]
    pub parent_name: String,
    // path
    #[serde(skip)]
    pub parent_id: String,
    // path
    #[serde(skip)]
    pub name: String,
    // path
    #[serde(skip)]
    pub id: String,
    properties: T,
}
