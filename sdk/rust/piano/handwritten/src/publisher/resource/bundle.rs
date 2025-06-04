mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// List resources of bundle
    ///
    /// Lists the resources of a given bundle.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Fbundle~2Fmembers)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_bundle_members(
        &self,
        params: &ListBundleMembersRequest<'_>,
    ) -> Result<PianoPaginated<crate::publisher::resource::ResourceListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/resource/bundle/members",
                self.endpoint
            ))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<crate::publisher::resource::ResourceListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
}
