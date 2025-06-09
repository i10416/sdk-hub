use serde::{Deserialize, Serialize};

/// Request to get app details by app ID
#[derive(Debug, Serialize, Default)]
pub struct GetAppRequest {
    // aid is passed as query parameter and is handled by the API client
}

impl GetAppRequest {
    /// Create a new get app request
    pub fn new() -> Self {
        Self::default()
    }
}

/// Response wrapper for app get operations
#[derive(Debug, Deserialize, Clone)]
pub struct AppResult {
    pub app: App,
}

/// A Piano application object
#[derive(Debug, Deserialize, Clone)]
pub struct App {
    /// The application ID
    pub aid: String,
    /// The default language
    pub default_lang: String,
    /// The email language
    pub email_lang: String,
    /// The application details (nullable)
    pub details: Option<String>,
    /// Email address associated with this app
    pub email: String,
    /// The application name
    pub name: String,
    /// The user token provider
    pub user_provider: UserProvider,
    /// The application website
    pub url: String,
    /// Primary image displayed within the dashboard
    pub logo1: String,
    /// Secondary image displayed within the ticket (nullable)
    pub logo2: Option<String>,
    /// Current state of the app
    pub state: AppState,
    /// The app's private key (empty for security reasons)
    pub private_key: String,
    /// API token (empty for security reasons)
    pub api_token: String,
}

/// The user token provider enum
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserProvider {
    TinypassGo,
    TinypassAccounts,
    PublisherUserRef,
    Janrain,
    Conde,
    Gigya,
    PianoId,
    PianoIdLite,
    #[serde(other)]
    Other,
}

/// Current state of the app enum
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AppState {
    Active,
    Inactive,
    Suspended,
    Declined,
    New,
    #[serde(other)]
    Other,
}

impl App {
    /// Get the application ID
    pub fn aid(&self) -> &str {
        &self.aid
    }

    /// Get the application name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the application email
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get the application URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the application details
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    /// Get the default language
    pub fn default_lang(&self) -> &str {
        &self.default_lang
    }

    /// Get the email language
    pub fn email_lang(&self) -> &str {
        &self.email_lang
    }

    /// Get the user provider
    pub fn user_provider(&self) -> &UserProvider {
        &self.user_provider
    }

    /// Get the primary logo
    pub fn logo1(&self) -> &str {
        &self.logo1
    }

    /// Get the secondary logo
    pub fn logo2(&self) -> Option<&str> {
        self.logo2.as_deref()
    }

    /// Get the app state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Check if the app is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, AppState::Active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PianoResponse;

    #[test]
    fn test_app_deserialization() {
        let json = serde_json::json!({
            "aid": "BbSvCkpwsu",
            "default_lang": "en_US",
            "email_lang": "en_US",
            "details": null,
            "email": "test@example.com",
            "name": "Test App",
            "user_provider": "piano_id",
            "url": "https://example.com/",
            "logo1": "/path/to/logo1.png",
            "logo2": null,
            "state": "active",
            "private_key": "",
            "api_token": ""
        });

        let app: App = serde_json::from_value(json).expect("Failed to deserialize app");
        assert_eq!(app.aid(), "BbSvCkpwsu");
        assert_eq!(app.name(), "Test App");
        assert_eq!(app.email(), "test@example.com");
        assert_eq!(app.url(), "https://example.com/");
        assert_eq!(app.details(), None);
        assert!(app.is_active());
        assert!(matches!(app.user_provider(), UserProvider::PianoId));
    }

    #[test]
    fn test_app_with_details() {
        let json = serde_json::json!({
            "aid": "test123",
            "default_lang": "en_US",
            "email_lang": "en_US",
            "details": "Test application details",
            "email": "test@example.com",
            "name": "Test App",
            "user_provider": "tinypass_go",
            "url": "https://example.com/",
            "logo1": "/path/to/logo1.png",
            "logo2": "/path/to/logo2.png",
            "state": "inactive",
            "private_key": "",
            "api_token": ""
        });

        let app: App =
            serde_json::from_value(json).expect("Failed to deserialize app with details");
        assert_eq!(app.details(), Some("Test application details"));
        assert_eq!(app.logo2(), Some("/path/to/logo2.png"));
        assert!(!app.is_active());
        assert!(matches!(app.state(), AppState::Inactive));
        assert!(matches!(app.user_provider(), UserProvider::TinypassGo));
    }

    #[test]
    fn sanity_check_get_app_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<AppResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize app get response: {:?}",
            value.err()
        );

        let response = value.unwrap();
        match response {
            PianoResponse::Succeed(data) => {
                let app = &data.app;
                assert_eq!(app.aid(), "***MASKED***");
                assert_eq!(app.name(), "***MASKED***");
                assert_eq!(app.email(), "***MASKED***");
                assert!(app.is_active());
                assert!(matches!(app.user_provider(), UserProvider::PianoId));
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
