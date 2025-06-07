mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// Create a resource tag
    ///
    /// Creates a resource tag.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fresource~2Ftag~2Fcreate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_resource_tag(
        &self,
        req: &CreateResourceTagRequest<'_>,
    ) -> Result<ResourceTag, crate::Error> {
        let result = self
            .client
            .post(format!("{}/publisher/resource/tag/create", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ResourceTagResult>>()
            .await?
            .value()?;
        Ok(result.resource_tag)
    }

    /// Get a resource tag by ID
    ///
    /// Returns a resource tag selected by ID.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Ftag~2Fget)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_resource_tag(
        &self,
        resource_tag_id: &str,
    ) -> Result<Option<ResourceTag>, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/resource/tag/get", self.endpoint))
            .query(&[
                ("aid", &self.app_id),
                ("resource_tag_id", &resource_tag_id.to_string()),
            ])
            .send()
            .await?
            .json::<PianoResponse<ResourceTagResult>>()
            .await?
            .maybe_value()?;
        Ok(result.map(|r| r.resource_tag))
    }

    /// List resource tags
    ///
    /// Lists resource tags with pagination and filtering options.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Ftag~2Flist)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_resource_tags(
        &self,
        params: &ListResourceTagRequest,
    ) -> Result<PianoPaginated<ResourceTagListResult>, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/resource/tag/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ResourceTagListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Delete a resource tag
    ///
    /// Deletes a resource tag.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fresource~2Ftag~2Fdelete)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_resource_tag(
        &self,
        req: &DeleteResourceTagRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(format!("{}/publisher/resource/tag/delete", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// Attach tag to resource
    ///
    /// Attaches a resource tag to a resource.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fresource~2Ftag~2Fattach)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn attach_resource_tag(
        &self,
        req: &AttachResourceTagRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(format!("{}/publisher/resource/tag/attach", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// Detach tag from resource
    ///
    /// Detaches a resource tag from a resource.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fresource~2Ftag~2Fdetach)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn detach_resource_tag(
        &self,
        req: &DetachResourceTagRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(format!("{}/publisher/resource/tag/detach", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// List bundles for tags
    ///
    /// Returns a list of the resources using a given resource tag.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Ftag~2Fbundles)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_tag_bundles(
        &self,
        params: &ListTagBundlesRequest,
    ) -> Result<PianoPaginated<crate::publisher::resource::ResourceListResult>, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/resource/tag/bundles", self.endpoint))
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
