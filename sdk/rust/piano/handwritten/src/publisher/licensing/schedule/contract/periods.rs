mod schema;
pub use schema::*;

use crate::PianoAPI;

impl PianoAPI {
    /// set status to `Active` for started periods or `Activated` for not started periods
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn activate_schedule_period<'a>(
        &self,
        req: &ActivatePeriodRequest<'a>,
    ) -> Result<(), crate::Error> {
        let _ = self
            .client
            .post(format!(
                "{}/publisher/licensing/schedule/contract/periods/activate",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        todo!()
    }
}
