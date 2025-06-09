mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// List consent box configs
    ///
    /// Lists consent box configs in a given app with optional filtering by type and enabled status.
    ///
    /// # Arguments
    ///
    /// * `params` - The list consents request parameters
    ///
    /// # Returns
    ///
    /// Returns a paginated list of consent configurations.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fconsent~2Flist) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_consents<'a>(
        &self,
        params: &ListConsentsRequest<'a>,
    ) -> Result<PianoPaginated<ConsentListResult>, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/consent/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ConsentListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Get app's consent box config
    ///
    /// Returns the config of a given consent box selected by consent ID and AID.
    ///
    /// # Arguments
    ///
    /// * `params` - The get consent request parameters
    ///
    /// # Returns
    ///
    /// Returns the consent configuration.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fconsent~2Fget) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_consent<'a>(
        &self,
        params: &GetConsentRequest<'a>,
    ) -> Result<ConsentResult, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/consent/get", self.endpoint))
            .query(&[
                ("aid", &self.app_id),
                ("consent_id", &params.consent_id.to_string()),
            ])
            .send()
            .await?
            .json::<PianoResponse<ConsentResult>>()
            .await?
            .value()?;
        Ok(result)
    }
}
