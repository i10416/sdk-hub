use serde::{Deserialize, Serialize};

/// Request to list promotion codes
#[derive(Debug, Serialize, Default)]
pub struct ListPromotionCodeRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset from which to start returning results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Field to order by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Order direction (asc/desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<String>,
    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Filter by promo code state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<String>>,
}

impl<'a> ListPromotionCodeRequest<'a> {
    /// Create a new list promotion code request
    pub fn new(promotion_id: &'a str) -> Self {
        Self {
            promotion_id,
            limit: None,
            offset: None,
            order_by: None,
            order_direction: None,
            q: None,
            state: None,
        }
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

    /// Set the order by field
    pub fn with_order_by(mut self, order_by: &str) -> Self {
        self.order_by = Some(order_by.to_string());
        self
    }

    /// Set the order direction
    pub fn with_order_direction(mut self, order_direction: &str) -> Self {
        self.order_direction = Some(order_direction.to_string());
        self
    }

    /// Set the search query
    pub fn with_query(mut self, q: &str) -> Self {
        self.q = Some(q.to_string());
        self
    }

    /// Set the state filter
    pub fn with_state(mut self, state: Vec<String>) -> Self {
        self.state = Some(state);
        self
    }
}

/// Request to create a promotion code
#[derive(Debug, Serialize)]
pub struct CreatePromotionCodeRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
    /// The promo code itself
    pub code: &'a str,
}

impl<'a> CreatePromotionCodeRequest<'a> {
    /// Create a new create promotion code request
    pub fn new(promotion_id: &'a str, code: &'a str) -> Self {
        Self { promotion_id, code }
    }
}

/// Request to count promotion codes
#[derive(Debug, Serialize, Default)]
pub struct CountPromotionCodeRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<&'a str>,
    /// Filter by promo code state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}

impl<'a> CountPromotionCodeRequest<'a> {
    /// Create a new count promotion code request
    pub fn new(promotion_id: &'a str) -> Self {
        Self {
            promotion_id,
            q: None,
            state: None,
        }
    }

    /// Set the search query
    pub fn with_query(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }

    /// Set the state filter
    pub fn with_state(mut self, state: &'a str) -> Self {
        self.state = Some(state);
        self
    }
}

/// Request to delete a promotion code
#[derive(Debug, Serialize)]
pub struct DeletePromotionCodeRequest<'a> {
    /// The promotion code ID
    pub promotion_code_id: &'a str,
}

impl<'a> DeletePromotionCodeRequest<'a> {
    /// Create a new delete promotion code request
    pub fn new(promotion_code_id: &'a str) -> Self {
        Self { promotion_code_id }
    }
}

/// Promotion code object
#[derive(Debug, Deserialize, Clone)]
pub struct PromotionCode {
    promotion_code_id: String,
    promotion_id: String,
    app_id: String,
    code: String,
    state: String,
    uses: i32,
    create_date: i64,
    create_by: Option<String>,
    update_date: Option<i64>,
    update_by: Option<String>,
    use_date: Option<i64>,
    user_id: Option<String>,
}

impl PromotionCode {
    /// Get the promotion code ID
    pub fn promotion_code_id(&self) -> &str {
        &self.promotion_code_id
    }

    /// Get the promotion ID
    pub fn promotion_id(&self) -> &str {
        &self.promotion_id
    }

    /// Get the app ID
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get the code
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get the state
    pub fn state(&self) -> &str {
        &self.state
    }

    /// Get the number of uses
    pub fn uses(&self) -> i32 {
        self.uses
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

    /// Get the use date
    pub fn use_date(&self) -> Option<i64> {
        self.use_date
    }

    /// Get the user ID
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }
}

/// Response wrapper for promotion code operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct PromotionCodeResult {
    #[serde(alias = "PromoCode")]
    pub promo_code: PromotionCode,
}

/// Response for promotion code list operations
#[derive(Debug, Deserialize, Clone)]
pub struct PromotionCodeListResult {
    #[serde(alias = "promo_codes")]
    pub promo_codes: Vec<PromotionCode>,
}

/// Response for promotion code count operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct PromotionCodeCountResult {
    pub count: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_promotion_code_request_builder() {
        let request = ListPromotionCodeRequest::new("promo123")
            .with_limit(50)
            .with_offset(10)
            .with_order_by("code")
            .with_query("test search")
            .with_state(vec!["active".to_string(), "used".to_string()]);

        assert_eq!(request.promotion_id, "promo123");
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.order_by, Some("code".to_string()));
        assert_eq!(request.q, Some("test search".to_string()));
        assert_eq!(
            request.state,
            Some(vec!["active".to_string(), "used".to_string()])
        );
    }

    #[test]
    fn test_create_promotion_code_request() {
        let request = CreatePromotionCodeRequest::new("promo123", "SAVE10");
        assert_eq!(request.promotion_id, "promo123");
        assert_eq!(request.code, "SAVE10");
    }

    #[test]
    fn test_count_promotion_code_request() {
        let request = CountPromotionCodeRequest::new("promo123")
            .with_query("test")
            .with_state("active");

        assert_eq!(request.promotion_id, "promo123");
        assert_eq!(request.q, Some("test"));
        assert_eq!(request.state, Some("active"));
    }

    #[test]
    fn test_delete_promotion_code_request() {
        let request = DeletePromotionCodeRequest::new("code123");
        assert_eq!(request.promotion_code_id, "code123");
    }

    #[test]
    fn test_promotion_code_deserialization() {
        let json = serde_json::json!({
            "promotion_code_id": "12345",
            "promotion_id": "promo123",
            "app_id": "app123",
            "code": "SAVE10",
            "state": "active",
            "uses": 0,
            "create_date": 1640995200,
            "create_by": "user123",
            "update_date": 1641081600,
            "update_by": "user456",
            "use_date": null,
            "user_id": null
        });

        let code: PromotionCode =
            serde_json::from_value(json).expect("Failed to deserialize promotion code");
        assert_eq!(code.promotion_code_id(), "12345");
        assert_eq!(code.promotion_id(), "promo123");
        assert_eq!(code.code(), "SAVE10");
        assert_eq!(code.state(), "active");
        assert_eq!(code.uses(), 0);
    }
}
