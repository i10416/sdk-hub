use serde::{Deserialize, Serialize};

// Resource type
#[derive(Debug, Deserialize)]
pub struct Object<T> {
    pub id: String,
    pub properties: T,
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

#[derive(Debug, Serialize)]
pub struct Pagination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Association {
    #[serde(rename = "toObjectId")]
    to_object_id: usize,
    #[serde(rename = "associationTypes")]
    association_types: Vec<AssociationType>,
}

impl Association {
    pub fn to_object_id(&self) -> usize {
        self.to_object_id
    }
    pub fn association_types(&self) -> &Vec<AssociationType> {
        &self.association_types
    }
}
#[derive(Debug, Deserialize)]
pub enum AssociationTypeCategory {
    #[serde(rename = "USER_DEFINED")]
    UserDefined,
    #[serde(rename = "HUBSPOT_DEFINED")]
    HubSpotDefined,
}

#[derive(Debug, Deserialize)]
pub struct AssociationType {
    category: AssociationTypeCategory,
    #[serde(rename = "typeId")]
    type_id: usize,
    label: Option<String>,
}

impl AssociationType {
    pub fn category(&self) -> &AssociationTypeCategory {
        &self.category
    }
    pub fn type_id(&self) -> usize {
        self.type_id
    }
    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }
}
