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
