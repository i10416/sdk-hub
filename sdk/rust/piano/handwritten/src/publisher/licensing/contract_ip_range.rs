mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// List contract IP ranges
    ///
    /// Lists the IP ranges of a given contract.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Flicensing~2FcontractIpRange~2Flist)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_contract_ip_ranges(
        &self,
        params: &ListContractIpRangeRequest<'_>,
    ) -> Result<PianoPaginated<ContractIpRangeListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/contractIpRange/list",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ContractIpRangeListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Create contract IP range
    ///
    /// Creates an IP range for a given contract.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractIpRange~2Fcreate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_contract_ip_range(
        &self,
        req: &CreateContractIpRangeRequest<'_>,
    ) -> Result<ContractIpRange, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contractIpRange/create",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ContractIpRangeResult>>()
            .await?
            .value()?;
        Ok(result.contract_ip_range)
    }

    /// Update contract IP range
    ///
    /// Updates an IP range for a given contract.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractIpRange~2Fupdate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_contract_ip_range(
        &self,
        req: &UpdateContractIpRangeRequest<'_>,
    ) -> Result<ContractIpRange, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contractIpRange/update",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ContractIpRangeResult>>()
            .await?
            .value()?;
        Ok(result.contract_ip_range)
    }

    /// Remove contract IP range
    ///
    /// Removes an IP range from a given contract.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractIpRange~2Fremove)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn remove_contract_ip_range(
        &self,
        req: &RemoveContractIpRangeRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!(
                "{}/publisher/licensing/contractIpRange/remove",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }
}
