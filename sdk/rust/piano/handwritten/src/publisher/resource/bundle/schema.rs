use serde::Serialize;

/// Request to list bundle members
#[derive(Debug, Serialize, Default)]
pub struct ListBundleMembersRequest<'a> {
    /// The resource ID
    pub rid: &'a str,
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
}

impl<'a> ListBundleMembersRequest<'a> {
    /// Create a new list bundle members request
    pub fn new(rid: &'a str) -> Self {
        Self {
            rid,
            limit: None,
            offset: None,
            order_by: None,
            order_direction: None,
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
}
