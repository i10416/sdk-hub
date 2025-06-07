mod schema;
pub use schema::*;

use crate::{Pagination, PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_contract_user<'a>(
        &self,
        req: &ListContractUserRequest<'a>,
        pagination: Pagination,
    ) -> Result<PianoPaginated<ContractUserListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/contractUser/list",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .query(req)
            .query(&pagination)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<_>>>()
            .await?
            .value()?;
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_contract_user<'a>(
        &self,
        req: &CreateContractUserRequest<'a>,
    ) -> Result<ContractUser, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contractUser/create",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ContractUserResult>>()
            .await?
            .value()?;
        Ok(result.contract_user)
    }
}
