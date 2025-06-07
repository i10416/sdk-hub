mod schema;
pub use schema::*;

use crate::{Empty, Pagination, PianoAPI, PianoPaginated, PianoResponse};

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

    /// Update an existing contract user
    ///
    /// Updates a contract user's details such as email, first name, and last name.
    ///
    /// # Arguments
    ///
    /// * `req` - The update contract user request
    ///
    /// # Returns
    ///
    /// Returns the updated contract user on success.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractUser~2Fupdate) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_contract_user<'a>(
        &self,
        req: &UpdateContractUserRequest<'a>,
    ) -> Result<ContractUser, crate::Error> {
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/contractUser/update",
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

    /// Remove a contract user from a contract
    ///
    /// Removes a contract user from the specified contract.
    ///
    /// # Arguments
    ///
    /// * `req` - The remove contract user request
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful removal.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractUser~2Fremove) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn remove_contract_user<'a>(
        &self,
        req: &RemoveContractUserRequest<'a>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!(
                "{}/publisher/licensing/contractUser/remove",
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

    /// Revoke access of a contract user
    ///
    /// Revokes the access of a contract user without removing them from the contract.
    ///
    /// # Arguments
    ///
    /// * `req` - The revoke contract user request
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful revocation.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Flicensing~2FcontractUser~2Frevoke) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn revoke_contract_user<'a>(
        &self,
        req: &RevokeContractUserRequest<'a>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!(
                "{}/publisher/licensing/contractUser/revoke",
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
