use serde::{Deserialize, Serialize};

/// Request to list promotion terms
#[derive(Debug, Serialize, Default)]
pub struct ListPromotionTermRequest<'a> {
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
}

impl<'a> ListPromotionTermRequest<'a> {
    /// Create a new list promotion term request
    pub fn new(promotion_id: &'a str) -> Self {
        Self {
            promotion_id,
            limit: None,
            offset: None,
            order_by: None,
            order_direction: None,
            q: None,
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
}

/// Request to add term to promotion
#[derive(Debug, Serialize)]
pub struct AddPromotionTermRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
    /// The term ID
    pub term_id: &'a str,
}

impl<'a> AddPromotionTermRequest<'a> {
    /// Create a new add promotion term request
    pub fn new(promotion_id: &'a str, term_id: &'a str) -> Self {
        Self {
            promotion_id,
            term_id,
        }
    }
}

/// Request to delete term from promotion
#[derive(Debug, Serialize)]
pub struct DeletePromotionTermRequest<'a> {
    /// The promotion ID
    pub promotion_id: &'a str,
    /// The term ID
    pub term_id: &'a str,
}

impl<'a> DeletePromotionTermRequest<'a> {
    /// Create a new delete promotion term request
    pub fn new(promotion_id: &'a str, term_id: &'a str) -> Self {
        Self {
            promotion_id,
            term_id,
        }
    }
}

/// Term object (reused from main term definitions)
#[derive(Debug, Deserialize, Clone)]
pub struct Term {
    term_id: String,
    pub_id: String,
    app_id: String,
    #[serde(rename = "type")]
    term_type: String,
    name: String,
    description: Option<String>,
    deleted: bool,
    resource_id: Option<String>,
    create_date: i64,
    create_by: Option<String>,
    update_date: Option<i64>,
    update_by: Option<String>,
}

impl Term {
    /// Get the term ID
    pub fn term_id(&self) -> &str {
        &self.term_id
    }

    /// Get the publisher ID
    pub fn pub_id(&self) -> &str {
        &self.pub_id
    }

    /// Get the app ID
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get the term type
    pub fn term_type(&self) -> &str {
        &self.term_type
    }

    /// Get the term name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the term description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Check if the term is deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    /// Get the resource ID
    pub fn resource_id(&self) -> Option<&str> {
        self.resource_id.as_deref()
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

/// Response for promotion term list operations
#[derive(Debug, Deserialize, Clone)]
pub struct PromotionTermListResult {
    #[serde(alias = "terms")]
    pub terms: Vec<Term>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_promotion_term_request_builder() {
        let request = ListPromotionTermRequest::new("promo123")
            .with_limit(50)
            .with_offset(10)
            .with_order_by("name")
            .with_query("test search");

        assert_eq!(request.promotion_id, "promo123");
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.order_by, Some("name".to_string()));
        assert_eq!(request.q, Some("test search".to_string()));
    }

    #[test]
    fn test_add_promotion_term_request() {
        let request = AddPromotionTermRequest::new("promo123", "term456");
        assert_eq!(request.promotion_id, "promo123");
        assert_eq!(request.term_id, "term456");
    }

    #[test]
    fn test_delete_promotion_term_request() {
        let request = DeletePromotionTermRequest::new("promo123", "term456");
        assert_eq!(request.promotion_id, "promo123");
        assert_eq!(request.term_id, "term456");
    }

    #[test]
    fn test_term_deserialization() {
        let json = serde_json::json!({
            "term_id": "12345",
            "pub_id": "pub123",
            "app_id": "app123",
            "type": "subscription",
            "name": "Monthly Subscription",
            "description": "A monthly subscription term",
            "deleted": false,
            "resource_id": "resource123",
            "create_date": 1640995200,
            "create_by": "user123",
            "update_date": 1641081600,
            "update_by": "user456"
        });

        let term: Term = serde_json::from_value(json).expect("Failed to deserialize term");
        assert_eq!(term.term_id(), "12345");
        assert_eq!(term.app_id(), "app123");
        assert_eq!(term.name(), "Monthly Subscription");
        assert_eq!(term.term_type(), "subscription");
        assert!(!term.is_deleted());
    }
}
