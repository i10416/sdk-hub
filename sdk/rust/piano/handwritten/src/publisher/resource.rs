pub mod bundle;
mod schema;
pub mod tag;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// Get a resource by ID
    ///
    /// Returns a resource selected by app ID and resource ID.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Fget)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_resource(&self, rid: &str) -> Result<Option<Resource>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/resource/get", self.endpoint))
            .query(&[("aid", &self.app_id), ("rid", &rid.to_string())])
            .send()
            .await?
            .json::<PianoResponse<ResourceResult>>()
            .await?
            .maybe_value()?;
        Ok(result.map(|r| r.resource))
    }

    /// Create a new resource
    ///
    /// Creates a resource.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fresource~2Fcreate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_resource(
        &self,
        req: &CreateResourceRequest<'_>,
    ) -> Result<Resource, crate::Error> {
        let result = self
            .client
            .post(&format!("{}/publisher/resource/create", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ResourceResult>>()
            .await?
            .value()?;
        Ok(result.resource)
    }

    /// Update an existing resource
    ///
    /// Updates a resource and returns its Resource object.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fresource~2Fupdate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_resource(
        &self,
        req: &UpdateResourceRequest<'_>,
    ) -> Result<Resource, crate::Error> {
        let result = self
            .client
            .post(&format!("{}/publisher/resource/update", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<ResourceResult>>()
            .await?
            .value()?;
        Ok(result.resource)
    }

    /// Delete a resource
    ///
    /// Deletes a resource.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fresource~2Fdelete)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_resource(
        &self,
        req: &DeleteResourceRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!("{}/publisher/resource/delete", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// List resources
    ///
    /// Lists resources of a given app with pagination and filtering options.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Flist)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_resources<'a>(
        &self,
        params: &ListResourceRequest<'a>,
    ) -> Result<PianoPaginated<ResourceListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/resource/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ResourceListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Count resources
    ///
    /// Returns the count of resources for a given app.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Fcount)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn count_resources(&self) -> Result<i32, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/resource/count", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .send()
            .await?
            .json::<PianoResponse<ResourceCountResult>>()
            .await?
            .value()?;
        Ok(result.data)
    }

    /// Attach resource to fixed bundle
    ///
    /// Attaches one or more resources to a given fixed bundle.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Fattach)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn attach_resource<'a>(
        &self,
        req: &AttachResourceRequest<'a>,
    ) -> Result<(), crate::Error> {
        self.client
            .get(&format!("{}/publisher/resource/attach", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(req)
            .send()
            .await?;
        Ok(())
    }

    /// Detach resource from fixed bundle
    ///
    /// Detaches a resource from a given fixed bundle.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Fdetach)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn detach_resource(
        &self,
        req: &DetachResourceRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .get(&format!("{}/publisher/resource/detach", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(req)
            .send()
            .await?;
        Ok(())
    }

    /// List bundles containing resource
    ///
    /// Lists the bundles a given resource belongs to.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fresource~2Fbundles)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_resource_bundles(
        &self,
        params: &ListResourceBundlesRequest<'_>,
    ) -> Result<PianoPaginated<ResourceListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/resource/bundles", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ResourceListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
}
