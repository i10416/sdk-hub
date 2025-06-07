pub mod periods;
pub mod schedule;
mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_contracts(
        &self,
        params: &ListContractRequest,
    ) -> Result<PianoPaginated<ListContractResult>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/contract/list",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ListContractResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_contract<'a>(
        &self,
        req: &CreateContractRequest<'a>,
    ) -> Result<Contract, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contract/create",
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_contract<'a>(
        &self,
        req: &UpdateContractRequest<'a>,
    ) -> Result<Contract, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contract/create",
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
