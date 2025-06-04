use serde::{Deserialize, Serialize};

/// Request to list bundle members
#[derive(Debug, Serialize, Default)]
pub struct ListBundleMembersRequest<'a> {
    /// The resource ID (bundle ID)
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

/// Bundle member object (same as Resource)
#[derive(Debug, Deserialize, Clone)]
pub struct BundleMember {
    rid: String,
    aid: String,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    resource_url: Option<String>,
    external_id: Option<String>,
    disabled: bool,
    deleted: bool,
    #[serde(rename = "type")]
    resource_type: String,
    create_date: i64,
    update_date: Option<i64>,
    publish_date: Option<i64>,
    is_fbia_resource: bool,
}

impl BundleMember {
    /// Get the RID
    pub fn rid(&self) -> &str {
        &self.rid
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the resource type
    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Check if deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}

/// Response for bundle member list operations
#[derive(Debug, Deserialize, Clone)]
pub struct BundleMemberListResult {
    pub resources: Vec<BundleMember>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PianoPaginated, PianoResponse};

    #[test]
    fn test_list_bundle_members_request_builder() {
        let request = ListBundleMembersRequest::new("bundle123")
            .with_limit(50)
            .with_offset(10)
            .with_order_by("name");

        assert_eq!(request.rid, "bundle123");
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.order_by, Some("name".to_string()));
    }

    #[test]
    fn test_bundle_member_deserialization() {
        let json = serde_json::json!({
            "resource_id": "12345",
            "rid": "test_rid",
            "aid": "app123",
            "name": "Test Member",
            "description": "A test member",
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

        let member: BundleMember =
            serde_json::from_value(json).expect("Failed to deserialize bundle member");
        assert_eq!(member.rid(), "test_rid");
        assert_eq!(member.name(), "Test Member");
        assert_eq!(member.resource_type(), "article");
        assert!(!member.is_disabled());
        assert!(!member.is_deleted());
    }

    #[test]
    fn sanity_check_list_bundle_members_codec() {
        let snapshot = include_str!("./members.schema.snapshot.json");
        let value =
            serde_json::from_str::<PianoResponse<PianoPaginated<BundleMemberListResult>>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize bundle member list: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(paginated) => {
                assert_eq!(paginated.limit, 1);
                assert_eq!(paginated.offset, 0);
                assert!(paginated.total >= 0);
                assert!(paginated.count >= 0);

                if !paginated.value.resources.is_empty() {
                    let member = &paginated.value.resources[0];
                    assert_eq!(member.name(), "***MASKED***");
                    assert_eq!(member.rid(), "***MASKED***");
                }
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
