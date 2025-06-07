use serde::{Deserialize, Serialize};

/// Request to create a promotion
#[derive(Debug, Serialize)]
pub struct CreatePromotionRequest<'a> {
    /// The promotion name
    pub name: &'a str,
    /// The promotion description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

impl<'a> CreatePromotionRequest<'a> {
    /// Create a new promotion request
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            description: None,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

/// Request to update a promotion
#[derive(Debug, Serialize)]
pub struct UpdatePromotionRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
    /// The promotion name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The promotion description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

impl<'a> UpdatePromotionRequest<'a> {
    /// Create a new update promotion request
    pub fn new(promotion_id: &'a str) -> Self {
        Self {
            promotion_id,
            name: None,
            description: None,
        }
    }

    /// Set the name
    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

/// Request to delete a promotion
#[derive(Debug, Serialize)]
pub struct DeletePromotionRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
}

impl<'a> DeletePromotionRequest<'a> {
    /// Create a new delete promotion request
    pub fn new(promotion_id: &'a str) -> Self {
        Self { promotion_id }
    }
}

/// Request to list promotions
#[derive(Debug, Serialize, Default)]
pub struct ListPromotionRequest {
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset from which to start returning results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

impl ListPromotionRequest {
    /// Create a new list promotion request
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Request to count promotions
#[derive(Debug, Serialize, Default)]
pub struct CountPromotionRequest {
    /// Filter criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

impl CountPromotionRequest {
    /// Create a new count promotion request
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the filter
    pub fn with_filter(mut self, filter: &str) -> Self {
        self.filter = Some(filter.to_string());
        self
    }
}

/// Request to generate promotion codes
#[derive(Debug, Serialize)]
pub struct GeneratePromotionRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
    /// Number of codes to generate
    pub count: i32,
}

impl<'a> GeneratePromotionRequest<'a> {
    /// Create a new generate promotion request
    pub fn new(promotion_id: &'a str, count: i32) -> Self {
        Self {
            promotion_id,
            count,
        }
    }
}

/// Promotion object
#[derive(Debug, Deserialize, Clone)]
pub struct Promotion {
    promotion_id: String,
    name: String,
    description: Option<String>,
    create_date: i64,
    create_by: Option<String>,
    update_date: Option<i64>,
    update_by: Option<String>,
}

impl Promotion {
    /// Get the promotion ID
    pub fn promotion_id(&self) -> &str {
        &self.promotion_id
    }

    /// Get the promotion name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the promotion description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get the creation date
    pub fn create_date(&self) -> i64 {
        self.create_date
    }

    /// Get the creator
    pub fn create_by(&self) -> Option<&str> {
        self.create_by.as_deref()
    }

    /// Get the update date
    pub fn update_date(&self) -> Option<i64> {
        self.update_date
    }

    /// Get the updater
    pub fn update_by(&self) -> Option<&str> {
        self.update_by.as_deref()
    }
}

/// Response wrapper for promotion operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct PromotionResult {
    #[serde(alias = "Promotion")]
    pub promotion: Promotion,
}

/// Response for promotion list operations
#[derive(Debug, Deserialize, Clone)]
pub struct PromotionListResult {
    #[serde(alias = "promotions")]
    pub promotions: Vec<Promotion>,
}

/// Response for promotion count operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct PromotionCountResult {
    pub count: i32,
}

/// Response for promotion exists operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct PromotionExistsResult {
    pub exists: bool,
}

/// Response for generate promotion operations
#[derive(Debug, Deserialize, Clone)]
pub struct GeneratePromotionResult {
    pub codes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PianoResponse;

    #[test]
    fn test_create_promotion_request() {
        let request =
            CreatePromotionRequest::new("Test Promotion").with_description("Test description");

        assert_eq!(request.name, "Test Promotion");
        assert_eq!(request.description, Some("Test description"));
    }

    #[test]
    fn test_generate_promotion_request() {
        let request = GeneratePromotionRequest::new("promo123", 100);
        assert_eq!(request.promotion_id, "promo123");
        assert_eq!(request.count, 100);
    }

    #[test]
    fn sanity_check_list_promotions_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value = serde_json::from_str::<
            crate::PianoResponse<crate::PianoPaginated<PromotionListResult>>,
        >(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize promotion list: {:?}",
            value.err()
        );
        let response = value.unwrap();

        let paginated = response.value().unwrap();
        assert_eq!(paginated.limit, 1);
        assert_eq!(paginated.offset, 0);
        assert_eq!(paginated.total, 52);
        assert_eq!(paginated.count, 1);

        if !paginated.value.promotions.is_empty() {
            let promotion = &paginated.value.promotions[0];
            assert_eq!(promotion.name(), "***MASKED***");
            assert_eq!(promotion.promotion_id(), "***MASKED***");
        }
    }

    // #[test]
    // fn sanity_check_count_promotions_codec() {
    //     let snapshot = include_str!("./count.schema.snapshot.json");
    //     let value = serde_json::from_str::<crate::PianoResponse<i32>>(snapshot);

    //     assert!(value.is_ok(), "Failed to deserialize promotion count: {:?}", value.err());
    //     let response = value.unwrap();

    //     assert_eq!(response.code, 0);
    //     assert!(response.ts > 0);
    //     assert!(response.data >= 0);
    // }

    #[test]
    fn sanity_check_get_promotion_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<PromotionResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize promotion get: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.promotion.name(), "***MASKED***");
                assert_eq!(data.promotion.promotion_id(), "***MASKED***");
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
