/// Access management endpoints for Piano Publisher API.
///
/// This module provides functionality to manage user access rights in the Piano platform,
/// including granting, revoking, and checking access to content.
///
/// ## Example
///
/// ```rust
/// use piano_handwritten_api::{PianoAPI, publisher::access::*};
///
/// # async fn example() -> Result<(), piano_handwritten_api::Error> {
/// let api = PianoAPI::new("https://api-us.piano.io/api/v3", "your_app_id", "your_token");
///
/// // Grant access to a user
/// let grant_req = GrantAccessRequest::new("user_uid", "resource_id");
/// api.grant_access(&grant_req).await?;
///
/// // Check if user has access
/// let has_access = api.check_access("user_uid", "resource_id").await?;
///
/// // List user's access rights
/// let access_list = api.list_user_access("user_uid", &ListAccessRequest::new()).await?;
/// # Ok(())
/// # }
/// ```
mod schema;
pub use self::schema::*;

use crate::{Empty, PianoAPI, PianoPaginated, PianoRequest, PianoResponse};

impl PianoAPI {
    /// Grant access to a user for a specific resource.
    ///
    /// This endpoint grants access permissions to a user for accessing content or resources.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fuser~2Faccess~2Fgrant
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn grant_access(
        &self,
        params: &GrantAccessRequest<'_>,
    ) -> Result<AccessGrant, crate::Error> {
        let req = PianoRequest {
            aid: self.app_id.clone(),
            inner: params,
        };
        let result = self
            .client
            .post(&format!("{}/publisher/access/grant", self.endpoint))
            .form(&req)
            .send()
            .await?
            .json::<PianoResponse<AccessGrantResult>>()
            .await?
            .value()?;
        Ok(result.access_grant)
    }

    /// Revoke access from a user for a specific resource.
    ///
    /// This endpoint removes access permissions from a user for specific content or resources.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fuser~2Faccess~2Frevoke
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn revoke_access(
        &self,
        params: &RevokeAccessRequest<'_>,
    ) -> Result<(), crate::Error> {
        let req = PianoRequest {
            aid: self.app_id.clone(),
            inner: params,
        };
        self.client
            .post(&format!("{}/publisher/access/revoke", self.endpoint))
            .form(&req)
            .send()
            .await?
            .json::<PianoResponse<Empty>>()
            .await?
            .value()?;
        Ok(())
    }

    /// Check if a user has access to a specific resource.
    ///
    /// This endpoint verifies whether a user has valid access permissions for content or resources.
    /// Returns `true` if the user has access, `false` otherwise.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fuser~2Faccess~2Fcheck
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn check_access(&self, uid: &str, resource_id: &str) -> Result<bool, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/access/check", self.endpoint))
            .query(&[
                ("aid", &self.app_id),
                ("uid", &uid.to_string()),
                ("resource_id", &resource_id.to_string()),
            ])
            .send()
            .await?
            .json::<PianoResponse<AccessCheckResult>>()
            .await?
            .value()?;
        Ok(result.has_access)
    }

    /// List access rights for a specific user.
    ///
    /// This endpoint retrieves a paginated list of access rights granted to a user.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fuser~2Faccess~2Flist
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_user_access(
        &self,
        uid: &str,
        params: &ListAccessRequest<'_>,
    ) -> Result<PianoPaginated<ListAccessResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/access/list", self.endpoint))
            .query(&[("aid", &self.app_id), ("uid", &uid.to_string())])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ListAccessResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// List all access grants with optional filtering.
    ///
    /// This endpoint retrieves a paginated list of all access grants in the system.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fuser~2Faccess~2Flist_all
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_all_access(
        &self,
        params: &ListAllAccessRequest<'_>,
    ) -> Result<PianoPaginated<ListAccessResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/access/list_all", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ListAccessResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
}
