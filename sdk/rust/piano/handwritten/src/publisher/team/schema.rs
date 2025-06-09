use serde::{Deserialize, Serialize};

/// Request to list team members
#[derive(Debug, Serialize, Default)]
pub struct ListTeamRequest {
    /// Comma-separated list of permissions to filter team members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
}

impl ListTeamRequest {
    /// Create a new list team request
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the permissions filter
    pub fn with_permissions(mut self, permissions: &[Permission]) -> Self {
        self.permissions = Some(
            permissions
                .iter()
                .map(|p| p.as_str())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Response wrapper for team member list operations
#[derive(Debug, Deserialize, Clone)]
pub struct TeamMemberListResult {
    #[serde(rename = "team_members")]
    pub team_members: Vec<TeamMember>,
}

/// A team member object
#[derive(Debug, Deserialize, Clone)]
pub struct TeamMember {
    /// The user's first name
    pub first_name: String,
    /// The user's last name
    pub last_name: String,
    /// The user's personal name (name and surname ordered as per locale)
    pub personal_name: String,
    /// The user's email address
    pub email: String,
    /// The user's ID
    pub uid: String,
    /// The creation date (timestamp)
    pub create_date: i64,
    /// The last login timestamp (optional, may not be present for users who haven't logged in)
    #[serde(default)]
    pub last_login: Option<i64>,
    /// Whether the invitation is expired
    #[serde(default)]
    pub invitation_expired: bool,
    /// List of permissions
    pub permissions: Vec<Permission>,
}

impl TeamMember {
    /// Get the user's first name
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Get the user's last name
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Get the user's personal name
    pub fn personal_name(&self) -> &str {
        &self.personal_name
    }

    /// Get the user's email address
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get the user's ID
    pub fn uid(&self) -> &str {
        &self.uid
    }

    /// Get the creation date
    pub fn create_date(&self) -> i64 {
        self.create_date
    }

    /// Get the last login timestamp
    pub fn last_login(&self) -> Option<i64> {
        self.last_login
    }

    /// Check if the invitation is expired
    pub fn is_invitation_expired(&self) -> bool {
        self.invitation_expired
    }

    /// Get the list of permissions
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    CancelSubscription,
    EditTemplates,
    FinancialReports,
    ManageApi,
    ManageBusiness,
    ManageCheckoutFlows,
    ManageComposer,
    ManageCookieManager,
    ManageCxenseSite,
    ManageEspAccount,
    ManageEspSite,
    ManageGlobalCustomFields,
    ManageLocalization,
    ManageNotifications,
    ManageOffers,
    ManagePaywalls,
    ManagePromotions,
    ManageResources,
    ManageSiteLicenses,
    ManageTeam,
    ManageTemplates,
    ManageTerms,
    ManageUpgradeOptions,
    ManageUsers,
    ManageWebhooks,
    MarketingFull,
    MarketingLimited,
    NonFinancialReports,
    PublisherAdmin,
    RefundPayment,
    RevokeAccess,
    ViewComposer,
    #[serde(other)]
    Other,
}

impl Permission {
    pub fn as_str(&self) -> &str {
        match self {
            Permission::CancelSubscription => "cancel_subscription",
            Permission::EditTemplates => "edit_templates",
            Permission::FinancialReports => "financial_reports",
            Permission::ManageApi => "manage_api",
            Permission::ManageBusiness => "manage_business",
            Permission::ManageCheckoutFlows => "manage_checkout_flows",
            Permission::ManageComposer => "manage_composer",
            Permission::ManageCookieManager => "manage_cookie_manager",
            Permission::ManageCxenseSite => "manage_cxense_site",
            Permission::ManageEspAccount => "manage_esp_account",
            Permission::ManageEspSite => "manage_esp_site",
            Permission::ManageGlobalCustomFields => "manage_global_custom_fields",
            Permission::ManageLocalization => "manage_localization",
            Permission::ManageNotifications => "manage_notifications",
            Permission::ManageOffers => "manage_offers",
            Permission::ManagePaywalls => "manage_paywalls",
            Permission::ManagePromotions => "manage_promotions",
            Permission::ManageResources => "manage_resources",
            Permission::ManageSiteLicenses => "manage_site_licenses",
            Permission::ManageTeam => "manage_team",
            Permission::ManageTemplates => "manage_templates",
            Permission::ManageTerms => "manage_terms",
            Permission::ManageUpgradeOptions => "manage_upgrade_options",
            Permission::ManageUsers => "manage_users",
            Permission::ManageWebhooks => "manage_webhooks",
            Permission::MarketingFull => "marketing_full",
            Permission::MarketingLimited => "marketing_limited",
            Permission::NonFinancialReports => "non_financial_reports",
            Permission::PublisherAdmin => "publisher_admin",
            Permission::RefundPayment => "refund_payment",
            Permission::RevokeAccess => "revoke_access",
            Permission::ViewComposer => "view_composer",
            Permission::Other => "other",
        }
    }
}

impl From<&str> for Permission {
    fn from(s: &str) -> Self {
        match s {
            "cancel_subscription" => Permission::CancelSubscription,
            "edit_templates" => Permission::EditTemplates,
            "financial_reports" => Permission::FinancialReports,
            "manage_api" => Permission::ManageApi,
            "manage_business" => Permission::ManageBusiness,
            "manage_checkout_flows" => Permission::ManageCheckoutFlows,
            "manage_composer" => Permission::ManageComposer,
            "manage_cookie_manager" => Permission::ManageCookieManager,
            "manage_cxense_site" => Permission::ManageCxenseSite,
            "manage_esp_account" => Permission::ManageEspAccount,
            "manage_esp_site" => Permission::ManageEspSite,
            "manage_global_custom_fields" => Permission::ManageGlobalCustomFields,
            "manage_localization" => Permission::ManageLocalization,
            "manage_notifications" => Permission::ManageNotifications,
            "manage_offers" => Permission::ManageOffers,
            "manage_paywalls" => Permission::ManagePaywalls,
            "manage_promotions" => Permission::ManagePromotions,
            "manage_resources" => Permission::ManageResources,
            "manage_site_licenses" => Permission::ManageSiteLicenses,
            "manage_team" => Permission::ManageTeam,
            "manage_templates" => Permission::ManageTemplates,
            "manage_terms" => Permission::ManageTerms,
            "manage_upgrade_options" => Permission::ManageUpgradeOptions,
            "manage_users" => Permission::ManageUsers,
            "manage_webhooks" => Permission::ManageWebhooks,
            "marketing_full" => Permission::MarketingFull,
            "marketing_limited" => Permission::MarketingLimited,
            "non_financial_reports" => Permission::NonFinancialReports,
            "publisher_admin" => Permission::PublisherAdmin,
            "refund_payment" => Permission::RefundPayment,
            "revoke_access" => Permission::RevokeAccess,
            "view_composer" => Permission::ViewComposer,
            _ => Permission::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PianoResponse;

    #[test]
    fn test_list_team_request_builder() {
        let request = ListTeamRequest::new()
            .with_permissions(&[Permission::ManageBusiness, Permission::ManagePromotions]);

        assert_eq!(
            request.permissions,
            Some("manage_business,manage_promotions".to_string())
        );
    }

    #[test]
    fn test_team_member_deserialization() {
        let json = serde_json::json!({
            "first_name": "John",
            "last_name": "Doe",
            "personal_name": "John Doe",
            "email": "john.doe@example.com",
            "uid": "12345",
            "create_date": 1640995200,
            "last_login": 1641081600,
            "invitation_expired": false,
            "permissions": ["manage_business", "manage_promotions"]
        });

        let team_member: TeamMember =
            serde_json::from_value(json).expect("Failed to deserialize team member");
        assert_eq!(team_member.first_name(), "John");
        assert_eq!(team_member.last_name(), "Doe");
        assert_eq!(team_member.email(), "john.doe@example.com");
        assert_eq!(team_member.uid(), "12345");
        assert!(!team_member.is_invitation_expired());
        assert_eq!(team_member.permissions().len(), 2);
        assert_eq!(team_member.last_login(), Some(1641081600));
    }

    #[test]
    fn test_team_member_deserialization_without_last_login() {
        let json = serde_json::json!({
            "first_name": "Jane",
            "last_name": "Smith",
            "personal_name": "Jane Smith",
            "email": "jane.smith@example.com",
            "uid": "67890",
            "create_date": 1640995200,
            "invitation_expired": true,
            "permissions": ["manage_business"]
        });

        let team_member: TeamMember = serde_json::from_value(json)
            .expect("Failed to deserialize team member without last_login");
        assert_eq!(team_member.first_name(), "Jane");
        assert_eq!(team_member.last_name(), "Smith");
        assert!(team_member.is_invitation_expired());
        assert_eq!(team_member.last_login(), None);
    }

    #[test]
    fn sanity_check_list_team_members_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<TeamMemberListResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize team member list: {:?}",
            value.err()
        );

        let response = value.unwrap();
        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.team_members.len(), 2);
                let first_member = &data.team_members[0];
                assert_eq!(first_member.first_name(), "***MASKED***");
                assert_eq!(first_member.email(), "***MASKED***");
                assert!(!first_member.is_invitation_expired());
                assert_eq!(first_member.permissions().len(), 3);
                assert_eq!(first_member.last_login(), Some(1749195551));

                let second_member = &data.team_members[1];
                assert!(second_member.is_invitation_expired());
                assert_eq!(second_member.last_login(), None);
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
