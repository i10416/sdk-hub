mod schema;
pub use schema::*;

use crate::{
    publisher::licensing::contract::{Contract, ContractResult},
    PianoAPI, PianoResponse,
};

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn activate_contract(
        &self,
        req: &ActivateContractRequest<'_>,
    ) -> Result<Contract, crate::Error> {
        let result = self
            .client
            .post(format!(
                "{}/publisher/licensing/contract/activate",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ContractResult>>()
            .await?
            .value()?;
        Ok(result.contract)
    }
}
