use serde::{Deserialize, Serialize};

/// Request to list consent box configs
#[derive(Debug, Serialize, Default)]
pub struct ListConsentsRequest<'a> {
    /// The consent box type ("registration" or "checkout")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConsentType>,
    /// Whether the user checked the consent box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Offset from which to start returning results
    pub offset: usize,
    /// Maximum index of returned results
    pub limit: usize,
    #[serde(skip)]
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> ListConsentsRequest<'a> {
    /// Create a new list consents request
    pub fn new(offset: usize, limit: usize) -> Self {
        Self {
            r#type: None,
            enabled: None,
            offset,
            limit,
            _lifetime: std::marker::PhantomData,
        }
    }

    /// Set the consent type filter
    pub fn with_type(mut self, consent_type: ConsentType) -> Self {
        self.r#type = Some(consent_type);
        self
    }

    /// Set the enabled filter
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

/// Request to get a specific consent box config
#[derive(Debug, Serialize)]
pub struct GetConsentRequest<'a> {
    pub consent_id: &'a str,
}

impl<'a> GetConsentRequest<'a> {
    /// Create a new get consent request
    pub fn new(consent_id: &'a str) -> Self {
        Self { consent_id }
    }
}

/// Response wrapper for consent list operations
#[derive(Debug, Deserialize, Clone)]
pub struct ConsentListResult {
    #[serde(rename = "Consents")]
    pub consents: Vec<Consent>,
}

/// Response wrapper for consent get operations
#[derive(Debug, Deserialize, Clone)]
pub struct ConsentResult {
    pub consent: Consent,
}

/// A consent box configuration object
#[derive(Debug, Deserialize, Clone)]
pub struct Consent {
    /// The consent ID
    pub consent_id: String,
    /// The field name
    pub field_name: String,
    /// The field ID
    pub field_id: String,
    /// The display text (HTML content)
    pub display_text: String,
    /// The error message
    pub error_message: String,
    /// The consent box type
    pub r#type: ConsentType,
    /// Whether the consent box is pre-checked
    pub pre_checked: bool,
    /// Whether the consent box is required
    pub required: bool,
    /// Whether the consent box is enabled
    pub enabled: bool,
    /// Whether the field ID is enabled
    pub field_id_enabled: bool,
}

/// Consent box type enum
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ConsentType {
    Registration,
    Checkout,
    #[serde(other)]
    Other,
}

impl Consent {
    /// Get the consent ID
    pub fn consent_id(&self) -> &str {
        &self.consent_id
    }

    /// Get the field name
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the field ID
    pub fn field_id(&self) -> &str {
        &self.field_id
    }

    /// Get the display text
    pub fn display_text(&self) -> &str {
        &self.display_text
    }

    /// Get the error message
    pub fn error_message(&self) -> &str {
        &self.error_message
    }

    /// Get the consent type
    pub fn consent_type(&self) -> &ConsentType {
        &self.r#type
    }

    /// Check if the consent box is pre-checked
    pub fn is_pre_checked(&self) -> bool {
        self.pre_checked
    }

    /// Check if the consent box is required
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Check if the consent box is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if the field ID is enabled
    pub fn is_field_id_enabled(&self) -> bool {
        self.field_id_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PianoPaginated;
    use crate::PianoResponse;

    #[test]
    fn test_list_consents_request_builder() {
        let request = ListConsentsRequest::new(0, 10)
            .with_type(ConsentType::Registration)
            .with_enabled(true);

        assert_eq!(request.offset, 0);
        assert_eq!(request.limit, 10);
        assert!(matches!(request.r#type, Some(ConsentType::Registration)));
        assert_eq!(request.enabled, Some(true));
    }

    #[test]
    fn test_get_consent_request_builder() {
        let request = GetConsentRequest::new("CBCVHXEHNWIXN");
        assert_eq!(request.consent_id, "CBCVHXEHNWIXN");
    }

    #[test]
    fn test_consent_deserialization() {
        let json = serde_json::json!({
            "consent_id": "CBCVHXEHNWIXN",
            "field_name": "Terms and Conditions",
            "field_id": "terms_and_conditions",
            "display_text": "I agree to the terms",
            "error_message": "You must agree to the terms",
            "type": "registration",
            "pre_checked": true,
            "required": true,
            "enabled": true,
            "field_id_enabled": false
        });

        let consent: Consent = serde_json::from_value(json).expect("Failed to deserialize consent");
        assert_eq!(consent.consent_id(), "CBCVHXEHNWIXN");
        assert_eq!(consent.field_name(), "Terms and Conditions");
        assert_eq!(consent.field_id(), "terms_and_conditions");
        assert!(consent.is_pre_checked());
        assert!(consent.is_required());
        assert!(consent.is_enabled());
        assert!(!consent.is_field_id_enabled());
        assert!(matches!(consent.consent_type(), ConsentType::Registration));
    }

    #[test]
    fn test_consent_type_deserialization() {
        assert!(matches!(
            serde_json::from_str::<ConsentType>("\"registration\"").unwrap(),
            ConsentType::Registration
        ));
        assert!(matches!(
            serde_json::from_str::<ConsentType>("\"checkout\"").unwrap(),
            ConsentType::Checkout
        ));
        assert!(matches!(
            serde_json::from_str::<ConsentType>("\"unknown\"").unwrap(),
            ConsentType::Other
        ));
    }

    #[test]
    fn sanity_check_list_consents_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value =
            serde_json::from_str::<PianoResponse<PianoPaginated<ConsentListResult>>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize consent list response: {:?}",
            value.err()
        );

        let response = value.unwrap();
        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.value.consents.len(), 1);
                let consent = &data.value.consents[0];
                assert_eq!(consent.consent_id(), "***MASKED***");
                assert_eq!(consent.field_name(), "***MASKED***");
                assert!(consent.is_enabled());
                assert!(matches!(consent.consent_type(), ConsentType::Registration));
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }

    #[test]
    fn sanity_check_get_consent_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<ConsentResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize consent get response: {:?}",
            value.err()
        );

        let response = value.unwrap();
        match response {
            PianoResponse::Succeed(data) => {
                let consent = &data.consent;
                assert_eq!(consent.consent_id(), "***MASKED***");
                assert_eq!(consent.field_name(), "***MASKED***");
                assert!(consent.is_enabled());
                assert!(matches!(consent.consent_type(), ConsentType::Registration));
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
