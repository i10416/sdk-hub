pub mod period;
mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoResponse};
impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_schedule(&self, schedule_id: &str) -> Result<Option<Schedule>, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/schedule/get", self.endpoint,))
            .query(&[("aid", &self.app_id)])
            .query(&[("schedule_id", schedule_id)])
            .send()
            .await?
            .json::<PianoResponse<ScheduleResult>>()
            .await?
            .maybe_value()?;
        Ok(result.map(|s| s.schedule))
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_schedule(&self, name: &str) -> Result<Schedule, crate::Error> {
        let result = self
            .client
            .post(format!("{}/publisher/schedule/create", self.endpoint,))
            .query(&[("aid", &self.app_id)])
            .form(&[("name", name)])
            .send()
            .await?
            .json::<PianoResponse<ScheduleResult>>()
            .await?
            .value()?;
        Ok(result.schedule)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_schedule(
        &self,
        schedule_id: &str,
        name: &str,
    ) -> Result<Schedule, crate::Error> {
        let result = self
            .client
            .post(format!("{}/publisher/schedule/update", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(&[("name", name), ("schedule_id", schedule_id)])
            .send()
            .await?
            .json::<PianoResponse<ScheduleResult>>()
            .await?
            .value()?;
        Ok(result.schedule)
    }
}
