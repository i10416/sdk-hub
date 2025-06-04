/// User management endpoints for Piano Publisher API.
///
/// This module provides functionality to manage users in the Piano platform,
/// including creating, updating, retrieving, and managing user data.
///
/// ## Example
///
/// ```rust
/// use piano_handwritten_api::{PianoAPI, publisher::user::*};
///
/// # async fn example() -> Result<(), piano_handwritten_api::Error> {
/// let api = PianoAPI::new("https://api-us.piano.io/api/v3", "your_app_id", "your_token");
///
/// // Get a user by UID
/// let user = api.get_user("user_uid").await?;
///
/// // Create a new user
/// let create_req = CreateUserRequest::new("user@example.com")
///     .with_first_name("John")
///     .with_last_name("Doe");
/// let new_user = api.create_user(&create_req).await?;
///
/// // Update user information
/// let update_req = UpdateUserRequest::new("user_uid")
///     .with_email("new_email@example.com");
/// let updated_user = api.update_user(&update_req).await?;
/// # Ok(())
/// # }
/// ```
mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoRequest, PianoResponse};

impl PianoAPI {
    /// Get user information by UID.
    ///
    /// This endpoint retrieves detailed information about a specific user.
    /// Returns `None` if the user is not found.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fuser~2Fget
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_user(&self, uid: &str) -> Result<Option<User>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/user/get", self.endpoint))
            .query(&[("aid", &self.app_id), ("uid", &uid.to_string())])
            .send()
            .await?
            .json::<PianoResponse<UserResult>>()
            .await?
            .maybe_value()?;
        Ok(result.map(|r| r.user))
    }

    /// Create a new user.
    ///
    /// This endpoint creates a new user in the Piano platform with the provided information.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fuser~2Fcreate
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_user(&self, params: &CreateUserRequest<'_>) -> Result<User, crate::Error> {
        let req = PianoRequest {
            aid: self.app_id.clone(),
            inner: params,
        };
        let result = self
            .client
            .post(&format!("{}/publisher/user/create", self.endpoint))
            .form(&req)
            .send()
            .await?
            .json::<PianoResponse<UserResult>>()
            .await?
            .value()?;
        Ok(result.user)
    }

    /// Update an existing user.
    ///
    /// This endpoint updates information for an existing user identified by UID.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fuser~2Fupdate
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_user(&self, params: &UpdateUserRequest<'_>) -> Result<User, crate::Error> {
        let req = PianoRequest {
            aid: self.app_id.clone(),
            inner: params,
        };
        let result = self
            .client
            .post(&format!("{}/publisher/user/update", self.endpoint))
            .form(&req)
            .send()
            .await?
            .json::<PianoResponse<UserResult>>()
            .await?
            .value()?;
        Ok(result.user)
    }

    /// List users with optional filtering and pagination.
    ///
    /// This endpoint retrieves a paginated list of users based on the provided criteria.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fuser~2Flist
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_users(
        &self,
        params: &ListUserRequest<'_>,
    ) -> Result<PianoPaginated<ListUserResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/user/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ListUserResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Search users by email or other criteria.
    ///
    /// This endpoint searches for users based on provided search criteria.
    ///
    /// Reference: https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fuser~2Fsearch
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn search_users(
        &self,
        params: &SearchUserRequest<'_>,
    ) -> Result<PianoPaginated<ListUserResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/user/search", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ListUserResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
}
