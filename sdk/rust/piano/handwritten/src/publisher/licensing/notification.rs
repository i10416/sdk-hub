mod schema;
pub use schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// List notifications for a licensee
    ///
    /// Returns an array of notifications about the events that happened or going to happen
    /// with any contract of the licensee.
    ///
    /// # Arguments
    ///
    /// * `params` - The list notifications request parameters
    ///
    /// # Returns
    ///
    /// Returns a paginated list of notifications.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Flicensing~2Fnotification~2Flist) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_notifications(
        &self,
        params: &ListNotificationRequest<'_>,
    ) -> Result<PianoPaginated<NotificationListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/notification/list",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<NotificationListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
}
