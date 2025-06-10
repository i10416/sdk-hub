use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ObjectEvent<T> {
    // Known Values: [DEAL, UNKNOWN]
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "objectId")]
    pub object_id: i32,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "objectTypeId")]
    pub object_type_id: String,
    pub properties: T,
    #[serde(default)]
    #[serde(rename = "is_deleted")]
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContactVersionEvent<T> {
    vid: u64,
    canonical_vid: i64,
    portal_id: u64,
    pub properties: T,
    #[serde(default)]
    is_contact: bool,
    associated_company: Company,
    associated_owner: Owner,
}

impl<T> ContactVersionEvent<T> {
    pub fn vid(&self) -> u64 {
        self.vid
    }

    pub fn canonical_vid(&self) -> i64 {
        self.canonical_vid
    }

    pub fn portal_id(&self) -> u64 {
        self.portal_id
    }

    pub fn properties(&self) -> &T {
        &self.properties
    }

    pub fn is_contact(&self) -> bool {
        self.is_contact
    }

    pub fn associated_company(&self) -> &Company {
        &self.associated_company
    }

    pub fn associated_owner(&self) -> &Owner {
        &self.associated_owner
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Company {
    company_id: u64,
    portal_id: u64,
    // properties: ...
}

impl Company {
    pub fn company_id(&self) -> u64 {
        self.company_id
    }

    pub fn portal_id(&self) -> u64 {
        self.portal_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Owner {
    email: String,
    first_name: String,
    last_name: String,
    hubspot_user_id: u64,
    // Known values: [PERSON]
    #[serde(rename = "type")]
    tpe: String,
}

impl Owner {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn hubspot_user_id(&self) -> u64 {
        self.hubspot_user_id
    }

    pub fn tpe(&self) -> &str {
        &self.tpe
    }
}

pub type Object<T> = ObjectEvent<T>;
