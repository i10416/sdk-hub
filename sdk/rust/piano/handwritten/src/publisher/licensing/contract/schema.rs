use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct ListContractRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
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
    contract_type: PianoContractType,
    name: String,
    seats_number: usize,
    is_hard_seats_limit_type: bool,
    rid: String,
    contract_is_active: bool,
}
impl Contract {
    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }
    pub fn rid(&self) -> &str {
        &self.rid
    }
    pub fn seats_number(&self) -> usize {
        self.seats_number
    }
    pub fn contract_name(&self) -> &str {
        &self.name
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
    use crate::{PianoResponse, PianoPaginated};

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
}
