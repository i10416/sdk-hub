use serde::{Deserialize, Serialize};

/// Request to list contract IP ranges
#[derive(Debug, Serialize, Default)]
pub struct ListContractIpRangeRequest<'a> {
    /// The contract ID
    pub contract_id: &'a str,
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

impl<'a> ListContractIpRangeRequest<'a> {
    /// Create a new list contract IP range request
    pub fn new(contract_id: &'a str) -> Self {
        Self {
            contract_id,
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

/// Request to create a contract IP range
#[derive(Debug, Serialize)]
pub struct CreateContractIpRangeRequest<'a> {
    /// The contract ID
    pub contract_id: &'a str,
    /// The IP range start
    pub ip_range_start: &'a str,
    /// The IP range end
    pub ip_range_end: &'a str,
    /// The IP range description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

impl<'a> CreateContractIpRangeRequest<'a> {
    /// Create a new create contract IP range request
    pub fn new(contract_id: &'a str, ip_range_start: &'a str, ip_range_end: &'a str) -> Self {
        Self {
            contract_id,
            ip_range_start,
            ip_range_end,
            description: None,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

/// Request to update a contract IP range
#[derive(Debug, Serialize)]
pub struct UpdateContractIpRangeRequest<'a> {
    /// The contract IP range ID
    pub contract_ip_range_id: &'a str,
    /// The IP range start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_range_start: Option<&'a str>,
    /// The IP range end
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_range_end: Option<&'a str>,
    /// The IP range description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

impl<'a> UpdateContractIpRangeRequest<'a> {
    /// Create a new update contract IP range request
    pub fn new(contract_ip_range_id: &'a str) -> Self {
        Self {
            contract_ip_range_id,
            ip_range_start: None,
            ip_range_end: None,
            description: None,
        }
    }

    /// Set the IP range start
    pub fn with_ip_range_start(mut self, ip_range_start: &'a str) -> Self {
        self.ip_range_start = Some(ip_range_start);
        self
    }

    /// Set the IP range end
    pub fn with_ip_range_end(mut self, ip_range_end: &'a str) -> Self {
        self.ip_range_end = Some(ip_range_end);
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

/// Request to remove a contract IP range
#[derive(Debug, Serialize)]
pub struct RemoveContractIpRangeRequest<'a> {
    /// The contract IP range ID
    pub contract_ip_range_id: &'a str,
}

impl<'a> RemoveContractIpRangeRequest<'a> {
    /// Create a new remove contract IP range request
    pub fn new(contract_ip_range_id: &'a str) -> Self {
        Self {
            contract_ip_range_id,
        }
    }
}

/// Contract IP range object
#[derive(Debug, Deserialize, Clone)]
pub struct ContractIpRange {
    contract_ip_range_id: String,
    contract_id: String,
    ip_range_start: String,
    ip_range_end: String,
    description: Option<String>,
    create_date: i64,
    create_by: Option<String>,
    update_date: Option<i64>,
    update_by: Option<String>,
}

impl ContractIpRange {
    /// Get the contract IP range ID
    pub fn contract_ip_range_id(&self) -> &str {
        &self.contract_ip_range_id
    }

    /// Get the contract ID
    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }

    /// Get the IP range start
    pub fn ip_range_start(&self) -> &str {
        &self.ip_range_start
    }

    /// Get the IP range end
    pub fn ip_range_end(&self) -> &str {
        &self.ip_range_end
    }

    /// Get the description
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

/// Response wrapper for contract IP range operations
#[derive(Debug, Deserialize, Clone)]
pub(super) struct ContractIpRangeResult {
    #[serde(alias = "ContractIpRange")]
    pub contract_ip_range: ContractIpRange,
}

/// Response for contract IP range list operations
#[derive(Debug, Deserialize, Clone)]
pub struct ContractIpRangeListResult {
    #[serde(alias = "contract_ip_ranges")]
    pub contract_ip_ranges: Vec<ContractIpRange>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_contract_ip_range_request() {
        let request =
            CreateContractIpRangeRequest::new("contract123", "192.168.1.1", "192.168.1.255")
                .with_description("Office network");

        assert_eq!(request.contract_id, "contract123");
        assert_eq!(request.ip_range_start, "192.168.1.1");
        assert_eq!(request.ip_range_end, "192.168.1.255");
        assert_eq!(request.description, Some("Office network"));
    }

    #[test]
    fn test_list_contract_ip_range_request() {
        let request = ListContractIpRangeRequest::new("contract123")
            .with_limit(50)
            .with_offset(10);

        assert_eq!(request.contract_id, "contract123");
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
    }
}
