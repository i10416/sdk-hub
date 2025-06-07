use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActivateContractRequest<'a> {
    contract_id: &'a str,
}
impl<'a> ActivateContractRequest<'a> {
    pub fn new(contract_id: &'a str) -> Self {
        Self { contract_id }
    }
}
