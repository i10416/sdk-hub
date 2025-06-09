mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// Get app details by application ID
    ///
    /// Returns an App object containing principal app properties for the given AID.
    /// The app ID is taken from the API client configuration.
    ///
    /// # Arguments
    ///
    /// * `_params` - The get app request parameters (currently unused since aid comes from client)
    ///
    /// # Returns
    ///
    /// Returns the app details.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fapp~2Fget) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_app(&self, _params: &GetAppRequest) -> Result<AppResult, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/app/get", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .send()
            .await?
            .json::<PianoResponse<AppResult>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Get app features by application ID
    ///
    /// Returns all app features enabled for the given app.
    /// The app ID is taken from the API client configuration.
    ///
    /// # Arguments
    ///
    /// * `_params` - The get app features request parameters (currently unused since aid comes from client)
    ///
    /// # Returns
    ///
    /// Returns the app features configuration.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fapp~2Ffeatures~2Fget) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_app_features(
        &self,
        _params: &GetAppFeaturesRequest,
    ) -> Result<AppFeaturesResult, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/app/features/get", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .send()
            .await?
            .json::<PianoResponse<AppFeaturesResult>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// List apps
    ///
    /// Returns an array of App objects for all your apps.
    ///
    /// # Arguments
    ///
    /// * `_params` - The list apps request parameters (currently unused)
    ///
    /// # Returns
    ///
    /// Returns a paginated list of apps.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fapp~2Flist) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_apps(
        &self,
        _params: &ListAppsRequest,
    ) -> Result<PianoPaginated<AppListResult>, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/app/list", self.endpoint))
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<AppListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
}
