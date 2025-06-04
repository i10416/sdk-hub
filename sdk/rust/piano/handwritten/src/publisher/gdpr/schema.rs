use serde::{Deserialize, Serialize};

/// Request to delete personal data for GDPR compliance
#[derive(Debug, Serialize)]
pub struct GdprDeleteRequest<'a> {
    /// The user ID
    pub uid: &'a str,
}

impl<'a> GdprDeleteRequest<'a> {
    /// Create a new GDPR delete request
    pub fn new(uid: &'a str) -> Self {
        Self { uid }
    }
}

/// Request to delete anonymous user profile for GDPR compliance
#[derive(Debug, Serialize)]
pub struct GdprDeleteAnonymousRequest<'a> {
    /// The anonymous user ID
    pub anon_uid: &'a str,
}

impl<'a> GdprDeleteAnonymousRequest<'a> {
    /// Create a new GDPR delete anonymous request
    pub fn new(anon_uid: &'a str) -> Self {
        Self { anon_uid }
    }
}

/// Request to export personal data for GDPR compliance
#[derive(Debug, Serialize)]
pub struct GdprExportRequest<'a> {
    /// The user ID
    pub uid: &'a str,
}

impl<'a> GdprExportRequest<'a> {
    /// Create a new GDPR export request
    pub fn new(uid: &'a str) -> Self {
        Self { uid }
    }
}

/// GDPR export response containing user data
#[derive(Debug, Deserialize, Clone)]
pub struct GdprExportResponse {
    /// User information
    pub user: Option<serde_json::Value>,
    /// User activities
    pub activities: Option<Vec<serde_json::Value>>,
    /// Additional data
    pub additional_data: Option<serde_json::Value>,
}

/// Response wrapper for GDPR export operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct GdprExportResponseResult {
    #[serde(alias = "EraseUserResponse")]
    pub erase_user_response: GdprExportResponse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdpr_delete_request() {
        let request = GdprDeleteRequest::new("user123");
        assert_eq!(request.uid, "user123");
    }

    #[test]
    fn test_gdpr_delete_anonymous_request() {
        let request = GdprDeleteAnonymousRequest::new("anon123");
        assert_eq!(request.anon_uid, "anon123");
    }

    #[test]
    fn test_gdpr_export_request() {
        let request = GdprExportRequest::new("user123");
        assert_eq!(request.uid, "user123");
    }
}
