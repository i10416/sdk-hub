use serde::{Deserialize, Serialize};

/// Request parameters for granting access to a user.
///
/// Contains the required parameters to grant access permissions.
#[derive(Debug, Serialize)]
pub struct GrantAccessRequest<'a> {
    /// User's unique identifier (required)
    pub uid: &'a str,
    /// Resource identifier (required)
    pub resource_id: &'a str,
    /// Type of resource (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<&'a str>,
    /// Access expiration date (UNIX timestamp) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    /// Custom parameters for the access grant (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<&'a str>,
}

impl<'a> GrantAccessRequest<'a> {
    /// Create a new access grant request with required parameters.
    pub fn new(uid: &'a str, resource_id: &'a str) -> Self {
        Self {
            uid,
            resource_id,
            resource_type: None,
            expires: None,
            custom_params: None,
        }
    }

    /// Set the resource type.
    pub fn with_resource_type(mut self, resource_type: &'a str) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Set the expiration timestamp.
    pub fn with_expires(mut self, expires: i64) -> Self {
        self.expires = Some(expires);
        self
    }

    /// Set custom parameters.
    pub fn with_custom_params(mut self, custom_params: &'a str) -> Self {
        self.custom_params = Some(custom_params);
        self
    }
}

/// Request parameters for revoking access from a user.
///
/// Contains the required parameters to revoke access permissions.
#[derive(Debug, Serialize)]
pub struct RevokeAccessRequest<'a> {
    /// User's unique identifier (required)
    pub uid: &'a str,
    /// Resource identifier (required)
    pub resource_id: &'a str,
    /// Type of resource (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<&'a str>,
}

impl<'a> RevokeAccessRequest<'a> {
    /// Create a new access revoke request with required parameters.
    pub fn new(uid: &'a str, resource_id: &'a str) -> Self {
        Self {
            uid,
            resource_id,
            resource_type: None,
        }
    }

    /// Set the resource type.
    pub fn with_resource_type(mut self, resource_type: &'a str) -> Self {
        self.resource_type = Some(resource_type);
        self
    }
}

/// Request parameters for listing user access rights.
#[derive(Debug, Serialize)]
pub struct ListAccessRequest<'a> {
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Filter by resource type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<&'a str>,
    /// Filter by status (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

impl<'a> Default for ListAccessRequest<'a> {
    fn default() -> Self {
        Self {
            limit: None,
            offset: None,
            resource_type: None,
            status: None,
        }
    }
}

impl<'a> ListAccessRequest<'a> {
    /// Create a new access list request.
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

    /// Filter by resource type.
    pub fn with_resource_type(mut self, resource_type: &'a str) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Filter by status.
    pub fn with_status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }
}

/// Request parameters for listing all access grants.
#[derive(Debug, Serialize)]
pub struct ListAllAccessRequest<'a> {
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Filter by user UID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<&'a str>,
    /// Filter by resource ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<&'a str>,
    /// Filter by resource type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<&'a str>,
    /// Filter by status (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

impl<'a> Default for ListAllAccessRequest<'a> {
    fn default() -> Self {
        Self {
            limit: None,
            offset: None,
            uid: None,
            resource_id: None,
            resource_type: None,
            status: None,
        }
    }
}

impl<'a> ListAllAccessRequest<'a> {
    /// Create a new list all access request.
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

    /// Filter by user UID.
    pub fn with_uid(mut self, uid: &'a str) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Filter by resource ID.
    pub fn with_resource_id(mut self, resource_id: &'a str) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Filter by resource type.
    pub fn with_resource_type(mut self, resource_type: &'a str) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Filter by status.
    pub fn with_status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }
}

/// Response wrapper for access grant operations.
#[derive(Debug, Deserialize, Clone)]
pub(super) struct AccessGrantResult {
    /// The access grant data - Piano API can return with different field names
    #[serde(alias = "AccessGrant", alias = "access_grant")]
    pub access_grant: AccessGrant,
}

/// Response for access check operations.
#[derive(Debug, Deserialize, Clone)]
pub(super) struct AccessCheckResult {
    /// Whether the user has access to the resource
    #[serde(alias = "hasAccess", alias = "has_access")]
    pub has_access: bool,
}

/// Response for access list operations.
#[derive(Debug, Deserialize, Clone)]
pub struct ListAccessResult {
    /// Array of access grants
    #[serde(alias = "AccessGrants", alias = "access_grants")]
    pub access_grants: Vec<AccessGrant>,
}

/// Represents an access grant in the Piano platform.
///
/// Contains information about user access permissions for specific resources.
#[derive(Debug, Deserialize, Clone)]
pub struct AccessGrant {
    /// Access grant's unique identifier
    access_grant_id: String,
    /// User's unique identifier
    uid: String,
    /// Resource identifier
    resource_id: String,
    /// Type of resource
    #[serde(default)]
    resource_type: Option<String>,
    /// Access grant status
    #[serde(default)]
    status: Option<String>,
    /// Timestamp when access was granted (UNIX timestamp)
    #[serde(default)]
    granted_date: Option<i64>,
    /// Timestamp when access expires (UNIX timestamp)
    #[serde(default)]
    expires_date: Option<i64>,
    /// Custom parameters as JSON string (optional)
    #[serde(default)]
    custom_params: Option<String>,
}

impl AccessGrant {
    /// Get the access grant's unique identifier.
    pub fn access_grant_id(&self) -> &str {
        &self.access_grant_id
    }

    /// Get the user's unique identifier.
    pub fn uid(&self) -> &str {
        &self.uid
    }

    /// Get the resource identifier.
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    /// Get the resource type.
    pub fn resource_type(&self) -> Option<&str> {
        self.resource_type.as_deref()
    }

    /// Get the access grant status.
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Get the timestamp when access was granted.
    pub fn granted_date(&self) -> Option<i64> {
        self.granted_date
    }

    /// Get the timestamp when access expires.
    pub fn expires_date(&self) -> Option<i64> {
        self.expires_date
    }

    /// Get the custom parameters.
    pub fn custom_params(&self) -> Option<&str> {
        self.custom_params.as_deref()
    }

    /// Check if the access grant is currently active (not expired).
    pub fn is_active(&self) -> bool {
        match (self.status.as_deref(), self.expires_date) {
            (Some("ACTIVE"), None) => true,
            (Some("ACTIVE"), Some(expires)) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                expires > now
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_access_request_builder() {
        let req = GrantAccessRequest::new("user123", "article456")
            .with_resource_type("article")
            .with_expires(1234567890)
            .with_custom_params(r#"{"source": "subscription"}"#);

        assert_eq!(req.uid, "user123");
        assert_eq!(req.resource_id, "article456");
        assert_eq!(req.resource_type, Some("article"));
        assert_eq!(req.expires, Some(1234567890));
        assert_eq!(req.custom_params, Some(r#"{"source": "subscription"}"#));
    }

    #[test]
    fn test_revoke_access_request_builder() {
        let req = RevokeAccessRequest::new("user123", "article456").with_resource_type("article");

        assert_eq!(req.uid, "user123");
        assert_eq!(req.resource_id, "article456");
        assert_eq!(req.resource_type, Some("article"));
    }

    #[test]
    fn test_access_grant_deserialization() {
        let json = serde_json::json!({
            "access_grant_id": "grant123",
            "uid": "user123",
            "resource_id": "article456",
            "resource_type": "article",
            "status": "ACTIVE",
            "granted_date": 1234567890,
            "expires_date": 1999999999
        });

        let grant: AccessGrant =
            serde_json::from_value(json).expect("Failed to deserialize access grant");
        assert_eq!(grant.access_grant_id(), "grant123");
        assert_eq!(grant.uid(), "user123");
        assert_eq!(grant.resource_id(), "article456");
        assert_eq!(grant.resource_type(), Some("article"));
        assert_eq!(grant.status(), Some("ACTIVE"));
        assert_eq!(grant.granted_date(), Some(1234567890));
        assert_eq!(grant.expires_date(), Some(1999999999));
    }

    #[test]
    fn test_access_grant_is_active() {
        let active_grant = AccessGrant {
            access_grant_id: "grant1".to_string(),
            uid: "user1".to_string(),
            resource_id: "resource1".to_string(),
            resource_type: Some("article".to_string()),
            status: Some("ACTIVE".to_string()),
            granted_date: Some(1234567890),
            expires_date: Some(2999999999), // Far future
            custom_params: None,
        };

        let expired_grant = AccessGrant {
            access_grant_id: "grant2".to_string(),
            uid: "user2".to_string(),
            resource_id: "resource2".to_string(),
            resource_type: Some("article".to_string()),
            status: Some("ACTIVE".to_string()),
            granted_date: Some(1234567890),
            expires_date: Some(1234567891), // Past date
            custom_params: None,
        };

        let inactive_grant = AccessGrant {
            access_grant_id: "grant3".to_string(),
            uid: "user3".to_string(),
            resource_id: "resource3".to_string(),
            resource_type: Some("article".to_string()),
            status: Some("REVOKED".to_string()),
            granted_date: Some(1234567890),
            expires_date: Some(2999999999),
            custom_params: None,
        };

        assert!(active_grant.is_active());
        assert!(!expired_grant.is_active());
        assert!(!inactive_grant.is_active());
    }

    #[test]
    fn test_access_check_result_deserialization() {
        let json = serde_json::json!({
            "has_access": true
        });

        let result: AccessCheckResult =
            serde_json::from_value(json).expect("Failed to deserialize access check result");
        assert!(result.has_access);
    }

    #[test]
    fn test_list_access_result_deserialization() {
        let json = serde_json::json!({
            "access_grants": [
                {
                    "access_grant_id": "grant1",
                    "uid": "user1",
                    "resource_id": "resource1",
                    "status": "ACTIVE"
                },
                {
                    "access_grant_id": "grant2",
                    "uid": "user2",
                    "resource_id": "resource2",
                    "status": "REVOKED"
                }
            ]
        });

        let result: ListAccessResult =
            serde_json::from_value(json).expect("Failed to deserialize list access result");
        assert_eq!(result.access_grants.len(), 2);
        assert_eq!(result.access_grants[0].access_grant_id(), "grant1");
        assert_eq!(result.access_grants[1].access_grant_id(), "grant2");
    }
}
