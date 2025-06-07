mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoResponse};

impl PianoAPI {
    /// Delete personal data
    ///
    /// Deletes a given user's personal data for GDPR compliance.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fgdpr~2Fdelete)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn gdpr_delete(&self, req: &GdprDeleteRequest<'_>) -> Result<(), crate::Error> {
        self.client
            .post(format!("{}/publisher/gdpr/delete", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// Delete anonymous user profile
    ///
    /// Deletes an anonymous user profile for GDPR compliance.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fgdpr~2FdeleteAnon)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn gdpr_delete_anonymous(
        &self,
        req: &GdprDeleteAnonymousRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(format!("{}/publisher/gdpr/deleteAnon", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// Get personal data export
    ///
    /// Returns information of a user and their activities from GDPR perspective.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fgdpr~2Fexport)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn gdpr_export(
        &self,
        req: &GdprExportRequest<'_>,
    ) -> Result<GdprExportResponse, crate::Error> {
        let result = self
            .client
            .post(format!("{}/publisher/gdpr/export", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<GdprExportResponseResult>>()
            .await?
            .value()?;
        Ok(result.erase_user_response)
    }
}
