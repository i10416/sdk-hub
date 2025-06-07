use serde::Serialize;

/// Request to list contract users for a domain
///
/// Lists contract users for a given domain of a given email domain contract.
///
/// # Reference
///
/// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Flicensing~2FcontractDomain~2FcontractUser~2Flist) for more details.
#[derive(Debug, Serialize)]
pub struct ListContractDomainUserRequest<'a> {
    /// The public ID of the contract
    pub contract_id: &'a str,
    /// The public ID of the contract domain
    pub contract_domain_id: &'a str,
    /// Field to order by (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<&'a str>,
    /// Order direction (asc/desc) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<&'a str>,
    /// Search query (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<&'a str>,
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

impl<'a> ListContractDomainUserRequest<'a> {
    /// Create a new list contract domain users request
    pub fn new(contract_id: &'a str, contract_domain_id: &'a str) -> Self {
        Self {
            contract_id,
            contract_domain_id,
            order_by: None,
            order_direction: None,
            q: None,
            limit: None,
            offset: None,
        }
    }

    /// Set the field to order by
    pub fn with_order_by(mut self, order_by: &'a str) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Set the order direction
    pub fn with_order_direction(mut self, order_direction: &'a str) -> Self {
        self.order_direction = Some(order_direction);
        self
    }

    /// Set the search query
    pub fn with_query(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }

    /// Set the maximum number of results to return
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_contract_domain_user_request_builder() {
        let req = ListContractDomainUserRequest::new("contract123", "domain456")
            .with_order_by("email")
            .with_order_direction("asc")
            .with_query("test")
            .with_limit(50)
            .with_offset(10);

        assert_eq!(req.contract_id, "contract123");
        assert_eq!(req.contract_domain_id, "domain456");
        assert_eq!(req.order_by, Some("email"));
        assert_eq!(req.order_direction, Some("asc"));
        assert_eq!(req.q, Some("test"));
        assert_eq!(req.limit, Some(50));
        assert_eq!(req.offset, Some(10));
    }
}
