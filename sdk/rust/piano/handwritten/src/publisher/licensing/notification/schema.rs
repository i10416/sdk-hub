use serde::{Deserialize, Serialize};

/// Request to list notifications for a licensee
///
/// Lists notifications about events that happened or are going to happen
/// with any contract of the licensee.
///
/// # Reference
///
/// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Flicensing~2Fnotification~2Flist) for more details.
#[derive(Debug, Serialize)]
pub struct ListNotificationRequest<'a> {
    /// The public ID of the licensee
    pub licensee_id: &'a str,
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

impl<'a> ListNotificationRequest<'a> {
    /// Create a new list notifications request
    pub fn new(licensee_id: &'a str) -> Self {
        Self {
            licensee_id,
            limit: None,
            offset: None,
        }
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

/// Notification information
///
/// Represents a notification about licensing events.
#[derive(Debug, Deserialize, Clone)]
pub struct Notification {
    /// Notification ID
    notification_id: String,
    /// Notification type
    #[serde(rename = "type")]
    notification_type: String,
    /// Notification message
    message: String,
    /// Creation date (Unix timestamp)
    create_date: i64,
    /// Contract ID associated with the notification
    #[serde(default)]
    contract_id: Option<String>,
    /// Licensee ID associated with the notification
    #[serde(default)]
    licensee_id: Option<String>,
}

impl Notification {
    /// Get the notification ID
    pub fn notification_id(&self) -> &str {
        &self.notification_id
    }

    /// Get the notification type
    pub fn notification_type(&self) -> &str {
        &self.notification_type
    }

    /// Get the notification message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the creation date (Unix timestamp)
    pub fn create_date(&self) -> i64 {
        self.create_date
    }

    /// Get the contract ID (if any)
    pub fn contract_id(&self) -> Option<&str> {
        self.contract_id.as_deref()
    }

    /// Get the licensee ID (if any)
    pub fn licensee_id(&self) -> Option<&str> {
        self.licensee_id.as_deref()
    }
}

/// List of notifications result
#[derive(Debug, Deserialize, Clone)]
pub struct NotificationListResult {
    /// Array of notifications
    #[serde(alias = "notifications")]
    pub notifications: Vec<Notification>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_notification_request_builder() {
        let req = ListNotificationRequest::new("licensee123")
            .with_limit(50)
            .with_offset(10);

        assert_eq!(req.licensee_id, "licensee123");
        assert_eq!(req.limit, Some(50));
        assert_eq!(req.offset, Some(10));
    }

    #[test]
    fn test_notification_deserialization() {
        let value = serde_json::json!({
            "notification_id": "notif123",
            "type": "CONTRACT_EXPIRING",
            "message": "Contract is expiring soon",
            "create_date": 1640995200,
            "contract_id": "contract123",
            "licensee_id": "licensee123"
        });

        let notification: Notification = serde_json::from_value(value).unwrap();
        assert_eq!(notification.notification_id(), "notif123");
        assert_eq!(notification.notification_type(), "CONTRACT_EXPIRING");
        assert_eq!(notification.message(), "Contract is expiring soon");
        assert_eq!(notification.create_date(), 1640995200);
        assert_eq!(notification.contract_id(), Some("contract123"));
        assert_eq!(notification.licensee_id(), Some("licensee123"));
    }
}
