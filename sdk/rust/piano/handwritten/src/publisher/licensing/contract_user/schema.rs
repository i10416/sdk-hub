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

/// Request to update an existing contract user
///
/// Updates a contract user's details.
///
/// # Reference
///
/// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractUser~2Fupdate) for more details.
#[derive(Debug, Serialize)]
pub struct UpdateContractUserRequest<'a> {
    /// The public ID of the contract
    pub contract_id: &'a str,
    /// The contract user's public ID
    pub contract_user_id: &'a str,
    /// The user's email address
    pub email: &'a str,
    /// The user's first name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// The user's last name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
}

impl<'a> UpdateContractUserRequest<'a> {
    /// Create a new update contract user request
    pub fn new(contract_id: &'a str, contract_user_id: &'a str, email: &'a str) -> Self {
        Self {
            contract_id,
            contract_user_id,
            email,
            first_name: None,
            last_name: None,
        }
    }

    /// Set the first name
    pub fn with_first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name
    pub fn with_last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }
}

/// Request to remove a contract user
///
/// Removes a contract user from a given contract.
///
/// # Reference
///
/// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractUser~2Fremove) for more details.
#[derive(Debug, Serialize)]
pub struct RemoveContractUserRequest<'a> {
    /// The public ID of the contract
    pub contract_id: &'a str,
    /// The contract user's public ID
    pub contract_user_id: &'a str,
}

impl<'a> RemoveContractUserRequest<'a> {
    /// Create a new remove contract user request
    pub fn new(contract_id: &'a str, contract_user_id: &'a str) -> Self {
        Self {
            contract_id,
            contract_user_id,
        }
    }
}

/// Request to revoke a contract user's access
///
/// Revokes access of a contract user.
///
/// # Reference
///
/// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractUser~2Frevoke) for more details.
#[derive(Debug, Serialize)]
pub struct RevokeContractUserRequest<'a> {
    /// The public ID of the contract
    pub contract_id: &'a str,
    /// The contract user's public ID
    pub contract_user_id: &'a str,
}

impl<'a> RevokeContractUserRequest<'a> {
    /// Create a new revoke contract user request
    pub fn new(contract_id: &'a str, contract_user_id: &'a str) -> Self {
        Self {
            contract_id,
            contract_user_id,
        }
    }
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractUserStatus {
    /// The user has an incorrect email.
    INVALID,
    /// The User has been invited but has not yet redeemed access.
    PENDING,
    /// The user has redeemed access
    ACTIVE,
    /// A member of your team has decided to revoke the user's access.
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
    /// The public ID of the contract
    pub contract_id: &'a str,
    /// Field to order by (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<&'a str>,
    /// Order direction (asc/desc) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<&'a str>,
    /// Search query (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<&'a str>,
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

impl<'a> ListContractUserRequest<'a> {
    /// Create a new list contract users request
    pub fn new(contract_id: &'a str) -> Self {
        Self {
            contract_id,
            order_by: None,
            order_direction: None,
            q: None,
            limit: None,
            offset: None,
        }
    }

    /// Set the field to order by
    pub fn with_order_by(mut self, order_by: &'a str) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Set the order direction
    pub fn with_order_direction(mut self, order_direction: &'a str) -> Self {
        self.order_direction = Some(order_direction);
        self
    }

    /// Set the search query
    pub fn with_query(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }

    /// Set the maximum number of results to return
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
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

    #[test]
    fn sanity_check_list_contract_user_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value =
            serde_json::from_str::<PianoResponse<PianoPaginated<ContractUserListResult>>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize contract user list: {:?}",
            value.err()
        );
    }
}
