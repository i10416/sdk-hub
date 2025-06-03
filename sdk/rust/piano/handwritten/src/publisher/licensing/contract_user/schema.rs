use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateContractUserRequest<'a> {
    pub contract_id: &'a str,
    pub email: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct ContractUserResult {
    #[serde(alias = "ContractUser")]
    pub contract_user: ContractUser,
}

#[derive(Debug, Deserialize, Clone)]

pub struct ContractUser {
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    contract_user_id: String,
    status: ContractUserStatus,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractUserStatus {
    INVALID,
    PENDING,
    ACTIVE,
    REVOKED,
}

impl ContractUser {
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn first_name(&self) -> &Option<String> {
        &self.first_name
    }
    pub fn last_name(&self) -> &Option<String> {
        &self.last_name
    }
    pub fn contract_user_id(&self) -> &str {
        &self.contract_user_id
    }
    pub fn status(&self) -> &ContractUserStatus {
        &self.status
    }
}

#[derive(Debug, Serialize)]
pub struct ListContractUserRequest<'a> {
    pub contract_id: &'a str,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractUserListResult {
    #[serde(rename = "ContractUserList")]
    pub contract_user_list: Vec<ContractUser>,
}

#[cfg(test)]
mod tests {
    use crate::{PianoPaginated, PianoResponse};

    use super::ContractUserListResult;

    #[test]
    fn sanity_check_list_deserialization() {
        let value = serde_json::json!({
          "code" : 0,
          "ts" : 1748945691,
          "limit" : 100,
          "offset" : 0,
          "total" : 1,
          "count" : 1,
          "ContractUserList" : [ {
            "contract_user_id" : "XXXXXXXXXX",
            "status" : "REVOKED",
            "email" : "john@example.com",
            "first_name" : null,
            "last_name" : null
          } ]
        });
        let result =
            serde_json::from_value::<PianoResponse<PianoPaginated<ContractUserListResult>>>(value);
        assert!(result.is_ok())
    }
}
