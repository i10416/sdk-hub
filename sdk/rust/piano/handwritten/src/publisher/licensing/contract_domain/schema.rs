use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ContractDomainListResult {
    #[serde(rename = "ContractDomainList")]
    pub contract_domains: Vec<ContractDomain>,
}

#[derive(Debug, Serialize)]
pub struct ListContractDomainRequest<'a> {
    contract_id: &'a str,
}

impl<'a> ListContractDomainRequest<'a> {
    pub fn new(contract_id: &'a str) -> Self {
        Self { contract_id }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateContractDomainRequest<'a> {
    contract_id: &'a str,
    contract_domain_value: &'a str,
}

#[derive(Debug, Serialize)]
pub struct UpdateContractDomainRequest<'a> {
    contract_domain_id: &'a str,
    contract_id: &'a str,
    contract_domain_value: &'a str,
}

#[derive(Debug, Serialize)]
pub struct RemoveContractDomainRequest<'a> {
    contract_domain_id: &'a str,
    contract_id: &'a str,
    contract_domain_value: &'a str,
}

impl<'a> CreateContractDomainRequest<'a> {
    pub fn contract_id(&self) -> &str {
        self.contract_id
    }
    pub fn contract_domain_value(&self) -> &str {
        self.contract_domain_value
    }
    pub fn new(contract_id: &'a str, contract_domain_value: &'a str) -> Self {
        Self {
            contract_id,
            contract_domain_value,
        }
    }
}

impl<'a> UpdateContractDomainRequest<'a> {
    pub fn contract_id(&self) -> &str {
        self.contract_id
    }
    pub fn contract_domain_value(&self) -> &str {
        self.contract_domain_value
    }
    pub fn new(contract_domain_id: &'a str, contract_id: &'a str, contract_domain_value: &'a str) -> Self {
        Self {
            contract_domain_id,
            contract_id,
            contract_domain_value,
        }
    }
}

impl<'a> RemoveContractDomainRequest<'a> {
    pub fn contract_id(&self) -> &str {
        self.contract_id
    }
    pub fn contract_domain_value(&self) -> &str {
        self.contract_domain_value
    }
    pub fn new(contract_domain_id: &'a str, contract_id: &'a str, contract_domain_value: &'a str) -> Self {
        Self {
            contract_domain_id,
            contract_id,
            contract_domain_value,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractDomainResult {
    #[serde(rename = "ContractDomain")]
    pub contract_domain: ContractDomain,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractDomain {
    pub contract_domain_id: String,
    pub contract_domain_value: String,
    pub status: String,
    pub contract_users_count: usize,
    pub active_contract_users_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PianoPaginated, PianoResponse};

    #[test]
    fn sanity_check_list_contract_domain_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<PianoPaginated<ContractDomainListResult>>>(
            snapshot,
        );

        assert!(
            value.is_ok(),
            "Failed to deserialize contract domain list: {:?}",
            value.err()
        );
    }
}
