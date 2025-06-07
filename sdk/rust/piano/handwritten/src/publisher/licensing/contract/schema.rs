use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct ListContractRequest<'a> {
    /// The public ID of the licensee (required)
    pub licensee_id: &'a str,
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

impl<'a> ListContractRequest<'a> {
    /// Create a new list contracts request
    pub fn new(licensee_id: &'a str) -> Self {
        Self {
            licensee_id,
            q: None,
            limit: None,
            offset: None,
        }
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

#[derive(Debug, Serialize)]
pub struct CreateContractRequest<'a> {
    pub licensee_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<&'a PianoContractType>,
    pub contract_name: &'a str,
    pub seats_number: usize,
    pub is_hard_seats_limit_type: bool,
    pub rid: &'a str,
    pub landing_page_url: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct UpdateContractRequest<'a> {
    pub licensee_id: &'a str,
    pub contract_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_description: Option<&'a str>,
    pub contract_type: &'a PianoContractType,
    pub contract_name: &'a str,
    pub seats_number: usize,
    pub is_hard_seats_limit_type: bool,
    pub rid: &'a str,
    pub landing_page_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<&'a str>,
}

/// Request to archive a contract
///
/// Archives a contract, making it inactive while preserving its data.
///
/// # Reference
///
/// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2Fcontract~2Farchive) for more details.
#[derive(Debug, Serialize)]
pub struct ArchiveContractRequest<'a> {
    /// The public ID of the contract
    pub contract_id: &'a str,
}

impl<'a> ArchiveContractRequest<'a> {
    /// Create a new archive contract request
    pub fn new(contract_id: &'a str) -> Self {
        Self { contract_id }
    }
}

/// Request to deactivate a contract
///
/// Deactivates a contract, temporarily disabling it.
///
/// # Reference
///
/// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2Fcontract~2Fdeactivate) for more details.
#[derive(Debug, Serialize)]
pub struct DeactivateContractRequest<'a> {
    /// The public ID of the contract
    pub contract_id: &'a str,
}

impl<'a> DeactivateContractRequest<'a> {
    /// Create a new deactivate contract request
    pub fn new(contract_id: &'a str) -> Self {
        Self { contract_id }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListContractResult {
    pub contracts: Vec<Contract>,
}

#[allow(nonstandard_style)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PianoContractType {
    SPECIFIC_EMAIL_ADDRESSES_CONTRACT,
    EMAIL_DOMAIN_CONTRACT,
    IP_RANGE_CONTRACT,
}

#[derive(Debug, Deserialize, Clone)]

pub(super) struct ContractResult {
    #[serde(alias = "Contract")]
    pub contract: Contract,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Contract {
    contract_id: String,
    aid: String,
    contract_type: PianoContractType,
    name: String,
    #[serde(default)]
    description: Option<String>,
    create_date: i64,
    #[serde(default)]
    landing_page_url: Option<String>,
    licensee_id: String,
    seats_number: usize,
    is_hard_seats_limit_type: bool,
    rid: String,
    #[serde(default)]
    schedule_id: Option<String>,
    contract_is_active: bool,
    #[serde(default)]
    contract_periods: Vec<ContractPeriod>,
    #[serde(default)]
    contract_conversions_count: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractPeriod {
    period_id: String,
    name: String,
    sell_date: i64,
    begin_date: i64,
    end_date: i64,
    status: SchedulePeriodStatus,
}

impl ContractPeriod {
    pub fn period_id(&self) -> &str {
        &self.period_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn sell_date(&self) -> i64 {
        self.sell_date
    }
    pub fn begin_date(&self) -> i64 {
        self.begin_date
    }
    pub fn end_date(&self) -> i64 {
        self.end_date
    }
    pub fn status(&self) -> &SchedulePeriodStatus {
        &self.status
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum SchedulePeriodStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "ENDED")]
    Ended,
}

impl Contract {
    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }
    pub fn aid(&self) -> &str {
        &self.aid
    }
    pub fn rid(&self) -> &str {
        &self.rid
    }
    pub fn licensee_id(&self) -> &str {
        &self.licensee_id
    }
    pub fn seats_number(&self) -> usize {
        self.seats_number
    }
    pub fn contract_name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn create_date(&self) -> i64 {
        self.create_date
    }
    pub fn landing_page_url(&self) -> Option<&str> {
        self.landing_page_url.as_deref()
    }
    pub fn schedule_id(&self) -> Option<&str> {
        self.schedule_id.as_deref()
    }
    pub fn contract_periods(&self) -> &[ContractPeriod] {
        &self.contract_periods
    }
    pub fn contract_conversions_count(&self) -> usize {
        self.contract_conversions_count
    }
    pub fn contract_type(&self) -> &PianoContractType {
        &self.contract_type
    }
    pub fn contract_is_active(&self) -> bool {
        self.contract_is_active
    }
    pub fn is_hard_seats_limit_type(&self) -> bool {
        self.is_hard_seats_limit_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PianoPaginated, PianoResponse};

    #[test]
    fn sanity_check_list_deserialization() {
        let value = serde_json::json!({
          "code" : 0,
          "ts" : 1748946794,
          "limit" : 100,
          "offset" : 0,
          "total" : 2,
          "count" : 2,
          "contracts" : [ {
            "contract_id" : "XXXXXXXXXX",
            "aid" : "XXXXXXXXXX",
            "name" : "Contract Name",
            "description" : null,
            "create_date" : 1710321904,
            "landing_page_url" : "landing_page_url",
            "licensee_id" : "XXXXXXXXXX",
            "seats_number" : 0,
            "is_hard_seats_limit_type" : false,
            "rid" : "XXXXXXXXXX",
            "schedule_id" : "XXXXXXXXXX",
            "contract_is_active" : true,
            "contract_type" : "SPECIFIC_EMAIL_ADDRESSES_CONTRACT",
            "contract_periods" : [ {
              "period_id" : "XXXXXXXXXX",
              "name" : "XXXXXXXXXX",
              "sell_date" : 1710216000,
              "begin_date" : 1710216000,
              "end_date" : 2122084800,
              "status" : "ACTIVE"
            }, {
              "period_id" : "XXXXXXXXXX",
              "name" : "XXXXXXXXXX",
              "sell_date" : 1744257600,
              "begin_date" : 2122084800,
              "end_date" : 2124590400,
              "status" : "ACTIVE"
            } ],
            "contract_conversions_count" : 12
          }]
        });
        let result = serde_json::from_value::<ListContractResult>(value);
        assert!(result.is_ok())
    }

    #[test]
    fn sanity_check_list_contract_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value =
            serde_json::from_str::<PianoResponse<PianoPaginated<ListContractResult>>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize contract list: {:?}",
            value.err()
        );
    }

    #[test]
    fn sanity_check_get_contract_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<ContractResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize contract get: {:?}",
            value.err()
        );
    }
}
