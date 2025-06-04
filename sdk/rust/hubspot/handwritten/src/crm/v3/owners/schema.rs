use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GetOwnerRequest {
    // query
    id_property: String,
    #[serde(default)]
    archived: bool,
}
impl GetOwnerRequest {
    pub fn new(id_property: &str, archived: bool) -> Self {
        Self {
            id_property: id_property.to_string(),
            archived,
        }
    }
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
    pub user_id: Option<usize>,
    #[serde(rename = "userIdIncludingInactive")]
    pub user_id_including_inactive: usize,
    pub email: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default)]
    pub teams: Vec<Team>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    #[serde(default)]
    pub primary: bool,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::Owner;

    #[test]
    fn sanity_check_owner_codec() {
        let value = serde_json::json!({
            "firstName": "DOE",
            "lastName": "JOHN",
            "createdAt": "2025-06-03T01:20:06.134Z",
            "archived": false,
            "id": "1234567",
            "type": "PERSON",
            "userId": Some(
                1234567,
            ),
            "userIdIncludingInactive": 1234567,
            "email": "test@example.com",
            "updatedAt": "2025-06-03T02:42:39.397Z",
            "teams": [],
        });
        let owner = serde_json::from_value::<Owner>(value);
        assert!(owner.is_ok())
    }
}
