pub mod schema;
pub use self::schema::*;

use crate::{Empty, PianoAPI, PianoResponse};

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn add_schedule_period<'a>(
        &self,
        req: &AddPeriodRequest<'a>,
    ) -> Result<Period, crate::Error> {
        let result = self
            .client
            .post(format!("{}/publisher/schedule/period/add", self.endpoint,))
            .query(&[("aid", &self.app_id)])
            .form(&req)
            .send()
            .await?
            .json::<PianoResponse<PeriodResult>>()
            .await?
            .value()?;
        Ok(result.period)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_schedule_period<'a>(
        &self,
        req: &UpdatePeriodRequest<'a>,
    ) -> Result<Period, crate::Error> {
        let result = self
            .client
            .post(format!(
                "{}/publisher/schedule/period/update",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(&req)
            .send()
            .await?
            .json::<PianoResponse<PeriodResult>>()
            .await?
            .value()?;
        Ok(result.period)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn remove_schedule_period(&self, period_id: &str) -> Result<(), crate::Error> {
        let _ = self
            .client
            .post(format!(
                "{}/publisher/schedule/period/remove",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(&[("period_id", period_id)])
            .send()
            .await?
            .json::<PianoResponse<Empty>>()
            .await?;
        Ok(())
    }
}
