mod schema;
pub use schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// List contract domain users
    ///
    /// Lists contract users for a given domain of a given email domain contract.
    ///
    /// # Arguments
    ///
    /// * `params` - The list contract domain users request parameters
    ///
    /// # Returns
    ///
    /// Returns a paginated list of contract users in the specified domain.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Flicensing~2FcontractDomain~2FcontractUser~2Flist) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_contract_domain_users(
        &self,
        params: &ListContractDomainUserRequest<'_>,
    ) -> Result<
        PianoPaginated<crate::publisher::licensing::contract_user::ContractUserListResult>,
        crate::Error,
    > {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/contractDomain/contractUser/list",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<
                PianoPaginated<crate::publisher::licensing::contract_user::ContractUserListResult>,
            >>()
            .await?
            .value()?;
        Ok(result)
    }
}
