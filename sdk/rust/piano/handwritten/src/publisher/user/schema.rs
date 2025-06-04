use serde::{Deserialize, Serialize};

/// Request parameters for creating a new user.
///
/// Contains the required and optional fields for user creation.
/// Email is the only required field.
#[derive(Debug, Serialize)]
pub struct CreateUserRequest<'a> {
    /// User's email address (required)
    pub email: &'a str,
    /// User's first name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// User's last name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// User's personal name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_name: Option<&'a str>,
    /// User's phone number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// User's custom parameters as JSON string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<&'a str>,
}

impl<'a> CreateUserRequest<'a> {
    /// Create a new user creation request with the required email field.
    pub fn new(email: &'a str) -> Self {
        Self {
            email,
            first_name: None,
            last_name: None,
            personal_name: None,
            phone: None,
            custom_params: None,
        }
    }

    /// Set the first name for the user.
    pub fn with_first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name for the user.
    pub fn with_last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Set the personal name for the user.
    pub fn with_personal_name(mut self, personal_name: &'a str) -> Self {
        self.personal_name = Some(personal_name);
        self
    }

    /// Set the phone number for the user.
    pub fn with_phone(mut self, phone: &'a str) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Set custom parameters for the user (as JSON string).
    pub fn with_custom_params(mut self, custom_params: &'a str) -> Self {
        self.custom_params = Some(custom_params);
        self
    }
}

/// Request parameters for updating an existing user.
///
/// Contains the UID of the user to update and optional fields to modify.
#[derive(Debug, Serialize)]
pub struct UpdateUserRequest<'a> {
    /// User's unique identifier (required)
    pub uid: &'a str,
    /// User's email address (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// User's first name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// User's last name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// User's personal name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_name: Option<&'a str>,
    /// User's phone number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// User's custom parameters as JSON string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<&'a str>,
}

impl<'a> UpdateUserRequest<'a> {
    /// Create a new user update request with the required UID.
    pub fn new(uid: &'a str) -> Self {
        Self {
            uid,
            email: None,
            first_name: None,
            last_name: None,
            personal_name: None,
            phone: None,
            custom_params: None,
        }
    }

    /// Set the email for the user.
    pub fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }

    /// Set the first name for the user.
    pub fn with_first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name for the user.
    pub fn with_last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Set the personal name for the user.
    pub fn with_personal_name(mut self, personal_name: &'a str) -> Self {
        self.personal_name = Some(personal_name);
        self
    }

    /// Set the phone number for the user.
    pub fn with_phone(mut self, phone: &'a str) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Set custom parameters for the user (as JSON string).
    pub fn with_custom_params(mut self, custom_params: &'a str) -> Self {
        self.custom_params = Some(custom_params);
        self
    }
}

/// Request parameters for listing users with filtering and pagination.
#[derive(Debug, Serialize)]
pub struct ListUserRequest<'a> {
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Search query string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<&'a str>,
    /// Filter by user status (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
    /// Order by field (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<&'a str>,
    /// Order direction (asc/desc) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<&'a str>,
}

impl<'a> Default for ListUserRequest<'a> {
    fn default() -> Self {
        Self {
            limit: None,
            offset: None,
            q: None,
            status: None,
            order_by: None,
            order_direction: None,
        }
    }
}

impl<'a> ListUserRequest<'a> {
    /// Create a new user list request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit for pagination.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the search query string.
    pub fn with_query(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }

    /// Set the status filter.
    pub fn with_status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the order by field.
    pub fn with_order_by(mut self, order_by: &'a str) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Set the order direction.
    pub fn with_order_direction(mut self, order_direction: &'a str) -> Self {
        self.order_direction = Some(order_direction);
        self
    }
}

/// Request parameters for searching users.
#[derive(Debug, Serialize)]
pub struct SearchUserRequest<'a> {
    /// Email to search for (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// UID to search for (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<&'a str>,
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

impl<'a> Default for SearchUserRequest<'a> {
    fn default() -> Self {
        Self {
            email: None,
            uid: None,
            limit: None,
            offset: None,
        }
    }
}

impl<'a> SearchUserRequest<'a> {
    /// Create a new user search request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Search by email.
    pub fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }

    /// Search by UID.
    pub fn with_uid(mut self, uid: &'a str) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Set the limit for pagination.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Response wrapper for single user operations.
#[derive(Debug, Deserialize, Clone)]
pub(super) struct UserResult {
    /// The user data - Piano API can return with different field names
    #[serde(alias = "User", alias = "user")]
    pub user: User,
}

/// Response for user list operations.
#[derive(Debug, Deserialize, Clone)]
pub struct ListUserResult {
    /// Array of users
    #[serde(alias = "Users", alias = "users")]
    pub users: Vec<User>,
}

/// Represents a Piano platform user.
///
/// Contains user information including personal details and platform-specific data.
#[derive(Debug, Deserialize, Clone)]
pub struct User {
    /// User's unique identifier
    uid: String,
    /// User's email address
    email: String,
    /// User's first name (optional)
    #[serde(default)]
    first_name: Option<String>,
    /// User's last name (optional)
    #[serde(default)]
    last_name: Option<String>,
    /// User's personal name (optional)
    #[serde(default)]
    personal_name: Option<String>,
    /// User's display name (optional)
    #[serde(default)]
    display_name: Option<String>,
    /// User's phone number (optional)
    #[serde(default)]
    phone: Option<String>,
    /// Timestamp when user was created (UNIX timestamp)
    #[serde(default)]
    create_date: Option<i64>,
    /// Timestamp when user was last updated (UNIX timestamp)
    #[serde(default)]
    update_date: Option<i64>,
    /// User's current status
    #[serde(default)]
    status: Option<String>,
    /// Custom parameters as JSON string (optional)
    #[serde(default)]
    custom_params: Option<String>,
}

impl User {
    /// Get the user's unique identifier.
    pub fn uid(&self) -> &str {
        &self.uid
    }

    /// Get the user's email address.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get the user's first name.
    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    /// Get the user's last name.
    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    /// Get the user's personal name.
    pub fn personal_name(&self) -> Option<&str> {
        self.personal_name.as_deref()
    }

    /// Get the user's phone number.
    pub fn phone(&self) -> Option<&str> {
        self.phone.as_deref()
    }

    /// Get the user's creation timestamp.
    pub fn create_date(&self) -> Option<i64> {
        self.create_date
    }

    /// Get the user's last update timestamp.
    pub fn update_date(&self) -> Option<i64> {
        self.update_date
    }

    /// Get the user's status.
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Get the user's custom parameters.
    pub fn custom_params(&self) -> Option<&str> {
        self.custom_params.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PianoResponse, PianoPaginated};

    #[test]
    fn test_create_user_request_builder() {
        let request = CreateUserRequest::new("user@example.com")
            .with_first_name("John")
            .with_last_name("Doe")
            .with_personal_name("Johnny");

        assert_eq!(request.email, "user@example.com");
        assert_eq!(request.first_name, Some("John"));
        assert_eq!(request.last_name, Some("Doe"));
        assert_eq!(request.personal_name, Some("Johnny"));
    }

    #[test]
    fn test_list_user_request_builder() {
        let request = ListUserRequest::new()
            .with_limit(50)
            .with_offset(10)
            .with_query("search term");

        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.q, Some("search term"));
    }

    #[test]
    fn test_user_deserialization() {
        let json = serde_json::json!({
            "uid": "12345",
            "email": "user@example.com",
            "first_name": "John",
            "last_name": "Doe",
            "personal_name": "Johnny",
            "create_date": 1640995200,
            "update_date": 1641081600
        });

        let user: User = serde_json::from_value(json).expect("Failed to deserialize user");
        assert_eq!(user.uid(), "12345");
        assert_eq!(user.email(), "user@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert_eq!(user.personal_name(), Some("Johnny"));
    }

    #[test]
    fn sanity_check_list_users_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<PianoPaginated<ListUserResult>>>(snapshot);
        
        assert!(value.is_ok(), "Failed to deserialize user list: {:?}", value.err());
        let response = value.unwrap();
        
        match response {
            PianoResponse::Succeed(paginated) => {
                assert_eq!(paginated.limit, 1);
                assert_eq!(paginated.offset, 0);
                assert!(paginated.total >= 0);
                assert!(paginated.count >= 0);
                
                if !paginated.value.users.is_empty() {
                    let user = &paginated.value.users[0];
                    assert_eq!(user.email(), "test@example.com");
                    assert_eq!(user.uid(), "***MASKED***");
                }
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }

    #[test]
    fn sanity_check_get_user_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<UserResult>>(snapshot);
        
        assert!(value.is_ok(), "Failed to deserialize user get: {:?}", value.err());
        let response = value.unwrap();
        
        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.user.email(), "test@example.com");
                assert_eq!(data.user.uid(), "***MASKED***");
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
