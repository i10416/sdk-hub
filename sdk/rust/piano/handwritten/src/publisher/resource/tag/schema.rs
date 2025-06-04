use serde::{Deserialize, Serialize};

/// Request to create a resource tag
#[derive(Debug, Serialize)]
pub struct CreateResourceTagRequest<'a> {
    /// The resource tag name
    pub name: &'a str,
    /// The resource tag description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

impl<'a> CreateResourceTagRequest<'a> {
    /// Create a new resource tag request
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            description: None,
        }
    }

    /// Set the resource tag description
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

/// Request to delete a resource tag
#[derive(Debug, Serialize)]
pub struct DeleteResourceTagRequest<'a> {
    /// The resource tag ID
    pub resource_tag_id: &'a str,
}

impl<'a> DeleteResourceTagRequest<'a> {
    /// Create a new delete resource tag request
    pub fn new(resource_tag_id: &'a str) -> Self {
        Self { resource_tag_id }
    }
}

/// Request to list resource tags
#[derive(Debug, Serialize, Default)]
pub struct ListResourceTagRequest {
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

impl ListResourceTagRequest {
    /// Create a new list resource tag request
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

/// Request to attach tag to resource
#[derive(Debug, Serialize)]
pub struct AttachResourceTagRequest<'a> {
    /// The resource ID
    pub rid: &'a str,
    /// The resource tag ID
    pub resource_tag_id: &'a str,
}

impl<'a> AttachResourceTagRequest<'a> {
    /// Create a new attach resource tag request
    pub fn new(rid: &'a str, resource_tag_id: &'a str) -> Self {
        Self {
            rid,
            resource_tag_id,
        }
    }
}

/// Request to detach tag from resource
#[derive(Debug, Serialize)]
pub struct DetachResourceTagRequest<'a> {
    /// The resource ID
    pub rid: &'a str,
    /// The resource tag ID
    pub resource_tag_id: &'a str,
}

impl<'a> DetachResourceTagRequest<'a> {
    /// Create a new detach resource tag request
    pub fn new(rid: &'a str, resource_tag_id: &'a str) -> Self {
        Self {
            rid,
            resource_tag_id,
        }
    }
}

/// Request to list tag bundles
#[derive(Debug, Serialize)]
pub struct ListTagBundlesRequest {
    /// The IDs of the included resources
    #[serde(serialize_with = "serialize_string_vec")]
    pub included_tag_id: Vec<String>,
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
    /// Disabled flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Resource type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub resource_type: Option<String>,
    /// Bundle type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<i32>,
}

impl ListTagBundlesRequest {
    /// Create a new list tag bundles request
    pub fn new(included_tag_id: Vec<String>) -> Self {
        Self {
            included_tag_id,
            limit: None,
            offset: None,
            order_by: None,
            order_direction: None,
            q: None,
            disabled: None,
            resource_type: None,
            bundle_type: None,
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

    /// Set the disabled filter
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set the resource type filter
    pub fn with_resource_type(mut self, resource_type: &str) -> Self {
        self.resource_type = Some(resource_type.to_string());
        self
    }

    /// Set the bundle type filter
    pub fn with_bundle_type(mut self, bundle_type: i32) -> Self {
        self.bundle_type = Some(bundle_type);
        self
    }
}

/// Resource tag object
#[derive(Debug, Deserialize, Clone)]
pub struct ResourceTag {
    resource_tag_id: String,
    app_id: String,
    name: String,
    description: Option<String>,
    create_date: i64,
    create_by: Option<String>,
    update_date: Option<i64>,
    update_by: Option<String>,
}

impl ResourceTag {
    /// Get the resource tag ID
    pub fn resource_tag_id(&self) -> &str {
        &self.resource_tag_id
    }

    /// Get the app ID
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get the resource tag name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the resource tag description
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

/// Response wrapper for resource tag operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct ResourceTagResult {
    #[serde(alias = "ResourceTag")]
    pub resource_tag: ResourceTag,
}

/// Response for resource tag list operations
#[derive(Debug, Deserialize, Clone)]
pub struct ResourceTagListResult {
    #[serde(alias = "resource_tags")]
    pub resource_tags: Vec<ResourceTag>,
}

// Helper function to serialize Vec<String> as comma-separated values
fn serialize_string_vec<S>(vec: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let concatenated = vec.join(",");
    serializer.serialize_str(&concatenated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_resource_tag_request_builder() {
        let request =
            CreateResourceTagRequest::new("Test Tag").with_description("Test description");

        assert_eq!(request.name, "Test Tag");
        assert_eq!(request.description, Some("Test description"));
    }

    #[test]
    fn test_list_tag_bundles_request_builder() {
        let request = ListTagBundlesRequest::new(vec!["tag1".to_string(), "tag2".to_string()])
            .with_limit(50)
            .with_resource_type("article")
            .with_disabled(false);

        assert_eq!(request.included_tag_id, vec!["tag1", "tag2"]);
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.resource_type, Some("article".to_string()));
        assert_eq!(request.disabled, Some(false));
    }

    #[test]
    fn test_resource_tag_deserialization() {
        let json = serde_json::json!({
            "resource_tag_id": "12345",
            "app_id": "app123",
            "name": "Test Tag",
            "description": "A test tag",
            "create_date": 1640995200,
            "create_by": "user123",
            "update_date": 1641081600,
            "update_by": "user456"
        });

        let tag: ResourceTag =
            serde_json::from_value(json).expect("Failed to deserialize resource tag");
        assert_eq!(tag.resource_tag_id(), "12345");
        assert_eq!(tag.app_id(), "app123");
        assert_eq!(tag.name(), "Test Tag");
        assert_eq!(tag.description(), Some("A test tag"));
    }
}
