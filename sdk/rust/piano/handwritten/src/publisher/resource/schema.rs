use serde::{Deserialize, Serialize};

/// Request to create a new resource
#[derive(Debug, Serialize)]
pub struct CreateResourceRequest<'a> {
    /// The resource ID
    pub rid: &'a str,
    /// The resource name
    pub name: &'a str,
    /// The resource type
    pub resource_type: &'a str,
    /// The resource description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The resource URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_url: Option<&'a str>,
    /// The resource image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<&'a str>,
    /// The resource external ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<&'a str>,
    /// Whether the resource is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The resource publish date (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<i64>,
}

impl<'a> CreateResourceRequest<'a> {
    /// Create a new resource request with required fields
    pub fn new(rid: &'a str, name: &'a str, resource_type: &'a str) -> Self {
        Self {
            rid,
            name,
            resource_type,
            description: None,
            resource_url: None,
            image_url: None,
            external_id: None,
            disabled: None,
            publish_date: None,
        }
    }

    /// Set the resource description
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the resource URL
    pub fn with_resource_url(mut self, resource_url: &'a str) -> Self {
        self.resource_url = Some(resource_url);
        self
    }

    /// Set the resource image URL
    pub fn with_image_url(mut self, image_url: &'a str) -> Self {
        self.image_url = Some(image_url);
        self
    }

    /// Set the resource external ID
    pub fn with_external_id(mut self, external_id: &'a str) -> Self {
        self.external_id = Some(external_id);
        self
    }

    /// Set whether the resource is disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set the resource publish date
    pub fn with_publish_date(mut self, publish_date: i64) -> Self {
        self.publish_date = Some(publish_date);
        self
    }
}

/// Request to update an existing resource
#[derive(Debug, Serialize)]
pub struct UpdateResourceRequest<'a> {
    /// The resource ID
    pub rid: &'a str,
    /// The resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The resource description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The resource URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_url: Option<&'a str>,
    /// The resource image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<&'a str>,
    /// The resource external ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<&'a str>,
    /// Whether the resource is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The resource publish date (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<i64>,
}

impl<'a> UpdateResourceRequest<'a> {
    /// Create a new update resource request
    pub fn new(rid: &'a str) -> Self {
        Self {
            rid,
            name: None,
            description: None,
            resource_url: None,
            image_url: None,
            external_id: None,
            disabled: None,
            publish_date: None,
        }
    }

    /// Set the resource name
    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the resource description
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the resource URL
    pub fn with_resource_url(mut self, resource_url: &'a str) -> Self {
        self.resource_url = Some(resource_url);
        self
    }

    /// Set the resource image URL
    pub fn with_image_url(mut self, image_url: &'a str) -> Self {
        self.image_url = Some(image_url);
        self
    }

    /// Set the resource external ID
    pub fn with_external_id(mut self, external_id: &'a str) -> Self {
        self.external_id = Some(external_id);
        self
    }

    /// Set whether the resource is disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set the resource publish date
    pub fn with_publish_date(mut self, publish_date: i64) -> Self {
        self.publish_date = Some(publish_date);
        self
    }
}

/// Request to delete a resource
#[derive(Debug, Serialize)]
pub struct DeleteResourceRequest<'a> {
    /// The resource ID
    pub rid: &'a str,
}

impl<'a> DeleteResourceRequest<'a> {
    /// Create a new delete resource request
    pub fn new(rid: &'a str) -> Self {
        Self { rid }
    }
}

/// Request to list resources
#[derive(Debug, Serialize, Default)]
pub struct ListResourceRequest<'a> {
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset from which to start returning results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Field to order by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<&'a str>,
    /// Order direction (asc/desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<&'a str>,
    /// Filter by resource type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<&'a str>,
    /// Filter by disabled status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<&'a str>,
}

impl<'a> ListResourceRequest<'a> {
    /// Create a new list resource request
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
    pub fn with_order_by(mut self, order_by: &'a str) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Set the order direction
    pub fn with_order_direction(mut self, order_direction: &'a str) -> Self {
        self.order_direction = Some(order_direction);
        self
    }

    /// Set the resource type filter
    pub fn with_resource_type(mut self, resource_type: &'a str) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Set the disabled filter
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set the search query
    pub fn with_query(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }
}

/// Request to attach resource to bundle
#[derive(Debug, Serialize)]
pub struct AttachResourceRequest<'a> {
    /// The resource bundle ID
    pub bundle_rid: &'a str,
    /// The RIDs of the fixed bundles containing this resource
    #[serde(serialize_with = "serialize_string_slice")]
    pub included_rid: &'a [&'a str],
}

impl<'a> AttachResourceRequest<'a> {
    /// Create a new attach resource request
    pub fn new(bundle_rid: &'a str, included_rid: &'a [&'a str]) -> Self {
        Self {
            bundle_rid,
            included_rid,
        }
    }
}

/// Request to detach resource from bundle
#[derive(Debug, Serialize)]
pub struct DetachResourceRequest<'a> {
    /// The resource ID
    pub rid: &'a str,
    /// The resource bundle ID
    pub bundle_rid: &'a str,
}

impl<'a> DetachResourceRequest<'a> {
    /// Create a new detach resource request
    pub fn new(rid: &'a str, bundle_rid: &'a str) -> Self {
        Self { rid, bundle_rid }
    }
}

/// Request to list resource bundles
#[derive(Debug, Serialize)]
pub struct ListResourceBundlesRequest<'a> {
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
    pub order_by: Option<&'a str>,
    /// Order direction (asc/desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<&'a str>,
}

impl<'a> ListResourceBundlesRequest<'a> {
    /// Create a new list resource bundles request
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
    pub fn with_order_by(mut self, order_by: &'a str) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Set the order direction
    pub fn with_order_direction(mut self, order_direction: &'a str) -> Self {
        self.order_direction = Some(order_direction);
        self
    }
}

/// Resource object
#[derive(Debug, Deserialize, Clone)]
pub struct Resource {
    rid: String,
    aid: String,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    purchase_url: Option<String>,
    resource_url: Option<String>,
    external_id: Option<String>,
    disabled: bool,
    deleted: bool,
    #[serde(rename = "type")]
    resource_type: String,
    create_date: i64,
    create_by: Option<String>,
    update_date: Option<i64>,
    update_by: Option<String>,
    publish_date: Option<i64>,
    bundle_type: Option<String>,
    #[serde(default)]
    is_fbia_resource: bool,
}

impl Resource {
    /// Get the resource RID
    pub fn rid(&self) -> &str {
        &self.rid
    }

    /// Get the app ID
    pub fn app_id(&self) -> &str {
        &self.aid
    }

    /// Get the resource name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the resource description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get the resource image URL
    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    /// Get the resource URL
    pub fn resource_url(&self) -> Option<&str> {
        self.resource_url.as_deref()
    }

    /// Get the external ID
    pub fn external_id(&self) -> Option<&str> {
        self.external_id.as_deref()
    }

    /// Check if the resource is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Check if the resource is deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    /// Get the resource type
    pub fn resource_type(&self) -> &str {
        &self.resource_type
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

    /// Get the publish date
    pub fn publish_date(&self) -> Option<i64> {
        self.publish_date
    }

    /// Get the bundle type
    pub fn bundle_type(&self) -> Option<&str> {
        self.bundle_type.as_deref()
    }

    /// Check if this is a FBIA resource
    pub fn is_fbia_resource(&self) -> bool {
        self.is_fbia_resource
    }
}

/// Response wrapper for resource operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct ResourceResult {
    #[serde(alias = "Resource")]
    pub resource: Resource,
}

/// Response for resource list operations
#[derive(Debug, Deserialize, Clone)]
pub struct ResourceListResult {
    #[serde(alias = "resources")]
    pub resources: Vec<Resource>,
}

/// Response for resource count operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct ResourceCountResult {
    pub data: i32,
}

fn serialize_string_slice<S>(slice: &&[&str], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(slice.len()))?;
    for element in slice.iter() {
        seq.serialize_element(element)?;
    }
    seq.end()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PianoPaginated, PianoResponse};

    #[test]
    fn test_create_resource_request_builder() {
        let request = CreateResourceRequest::new("test_rid", "Test Resource", "article")
            .with_description("Test description")
            .with_resource_url("https://example.com/article")
            .with_disabled(false);

        assert_eq!(request.rid, "test_rid");
        assert_eq!(request.name, "Test Resource");
        assert_eq!(request.resource_type, "article");
        assert_eq!(request.description, Some("Test description"));
        assert_eq!(request.resource_url, Some("https://example.com/article"));
        assert_eq!(request.disabled, Some(false));
    }

    #[test]
    fn test_list_resource_request_builder() {
        let request = ListResourceRequest::new()
            .with_limit(50)
            .with_offset(10)
            .with_resource_type("article")
            .with_disabled(false)
            .with_query("test search");

        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.resource_type, Some("article"));
        assert_eq!(request.disabled, Some(false));
        assert_eq!(request.q, Some("test search"));
    }

    #[test]
    fn test_resource_deserialization() {
        let json = serde_json::json!({
            "rid": "test_rid",
            "aid": "app123",
            "name": "Test Resource",
            "description": "A test resource",
            "image_url": null,
            "resource_url": "https://example.com",
            "external_id": "ext123",
            "disabled": false,
            "deleted": false,
            "type": "article",
            "create_date": 1640995200,
            "create_by": "user123",
            "update_date": 1641081600,
            "update_by": "user456",
            "publish_date": 1641168000,
            "bundle_type": null,
            "is_fbia_resource": false
        });

        let resource: Resource =
            serde_json::from_value(json).expect("Failed to deserialize resource");
        assert_eq!(resource.rid(), "test_rid");
        assert_eq!(resource.name(), "Test Resource");
        assert_eq!(resource.resource_type(), "article");
        assert!(!resource.is_disabled());
        assert!(!resource.is_deleted());
    }

    #[test]
    fn sanity_check_list_resources_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value =
            serde_json::from_str::<PianoResponse<PianoPaginated<ResourceListResult>>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize resource list: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(paginated) => {
                assert_eq!(paginated.limit, 1);
                assert_eq!(paginated.offset, 0);
                assert_eq!(paginated.total, 13);
                assert_eq!(paginated.count, 1);

                if !paginated.value.resources.is_empty() {
                    let resource = &paginated.value.resources[0];
                    assert_eq!(resource.name(), "***MASKED***");
                    assert_eq!(resource.rid(), "***MASKED***");
                }
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }

    #[test]
    fn sanity_check_count_resources_codec() {
        let snapshot = include_str!("./count.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<ResourceCountResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize resource count: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(data) => {
                assert!(data.data >= 0);
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }

    #[test]
    fn sanity_check_get_resource_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<ResourceResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize resource get: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.resource.name(), "***MASKED***");
                assert_eq!(data.resource.rid(), "***MASKED***");
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
