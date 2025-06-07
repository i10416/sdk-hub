pub mod periods;
pub mod schedule;
mod schema;
pub use self::schema::*;

use crate::{Empty, PianoAPI, PianoPaginated, PianoResponse};

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

    /// Get a contract by its ID
    ///
    /// Returns a contract by its public ID.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The public ID of the contract
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Contract))` if the contract is found, `Ok(None)` if not found, or an error.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Flicensing~2Fcontract~2Fget) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_contract(&self, contract_id: &str) -> Result<Option<Contract>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/contract/get",
                self.endpoint,
            ))
            .query(&[("aid", self.app_id.as_str()), ("contract_id", contract_id)])
            .send()
            .await?
            .json::<PianoResponse<ContractResult>>()
            .await?
            .maybe_value()?;
        Ok(result.map(|r| r.contract))
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
                "{}/publisher/licensing/contract/update",
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

    /// Archive a contract
    ///
    /// Archives a contract, making it inactive while preserving its data.
    ///
    /// # Arguments
    ///
    /// * `req` - The archive contract request
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful archiving.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2Fcontract~2Farchive) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn archive_contract<'a>(
        &self,
        req: &ArchiveContractRequest<'a>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!(
                "{}/publisher/licensing/contract/archive",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<Empty>>()
            .await?
            .value()?;
        Ok(())
    }

    /// Deactivate a contract
    ///
    /// Deactivates a contract, temporarily disabling it.
    ///
    /// # Arguments
    ///
    /// * `req` - The deactivate contract request
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful deactivation.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2Fcontract~2Fdeactivate) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn deactivate_contract<'a>(
        &self,
        req: &DeactivateContractRequest<'a>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!(
                "{}/publisher/licensing/contract/deactivate",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<Empty>>()
            .await?
            .value()?;
        Ok(())
    }
}
