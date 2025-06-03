use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ContractDomainListResult {
    #[serde(rename = "ContractDomainList")]
    pub contract_domains: Vec<ContractDomain>,
}

#[derive(Debug, Serialize)]
pub struct ListContractDomainRequest {
    contract_id: String,
}
impl ListContractDomainRequest {
    pub fn new(contract_id: &str) -> Self {
        Self {
            contract_id: contract_id.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateContractDomainRequest {
    contract_id: String,
    contract_domain_value: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateContractDomainRequest {
    contract_domain_id: String,
    contract_id: String,
    contract_domain_value: String,
}

#[derive(Debug, Serialize)]
pub struct RemoveContractDomainRequest {
    contract_domain_id: String,
    contract_id: String,
    contract_domain_value: String,
}

impl CreateContractDomainRequest {
    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }
    pub fn contract_domain_value(&self) -> &str {
        &self.contract_domain_value
    }
    pub fn new(contract_id: &str, contract_domain_value: &str) -> Self {
        Self {
            contract_id: contract_id.to_string(),
            contract_domain_value: contract_domain_value.to_string(),
        }
    }
}

impl UpdateContractDomainRequest {
    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }
    pub fn contract_domain_value(&self) -> &str {
        &self.contract_domain_value
    }
    pub fn new(contract_domain_id: &str, contract_id: &str, contract_domain_value: &str) -> Self {
        Self {
            contract_domain_id: contract_domain_id.to_string(),
            contract_id: contract_id.to_string(),
            contract_domain_value: contract_domain_value.to_string(),
        }
    }
}
impl RemoveContractDomainRequest {
    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }
    pub fn contract_domain_value(&self) -> &str {
        &self.contract_domain_value
    }
    pub fn new(contract_domain_id: &str, contract_id: &str, contract_domain_value: &str) -> Self {
        Self {
            contract_domain_id: contract_domain_id.to_string(),
            contract_id: contract_id.to_string(),
            contract_domain_value: contract_domain_value.to_string(),
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
