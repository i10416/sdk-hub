mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoResponse};

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
}
