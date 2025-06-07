mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_contract_domain<'a>(
        &self,
        params: &ListContractDomainRequest<'a>,
    ) -> Result<PianoPaginated<ContractDomainListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/contractDomain/list",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<_>>>()
            .await?
            .value()?;
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_contract_domain<'a>(
        &self,
        req: &CreateContractDomainRequest<'a>,
    ) -> Result<ContractDomain, crate::Error> {
        println!("{:?}", self.app_id);
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contractDomain/create",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ContractDomainResult>>()
            .await?
            .value()?;
        Ok(result.contract_domain)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_contract_domain<'a>(
        &self,
        req: &UpdateContractDomainRequest<'a>,
    ) -> Result<ContractDomain, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contractDomain/create",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ContractDomainResult>>()
            .await?
            .value()?;
        Ok(result.contract_domain)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn remove_contract_domain<'a>(
        &self,
        req: &RemoveContractDomainRequest<'a>,
    ) -> Result<ContractDomain, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contractDomain/create",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ContractDomainResult>>()
            .await?
            .value()?;
        Ok(result.contract_domain)
    }
}
