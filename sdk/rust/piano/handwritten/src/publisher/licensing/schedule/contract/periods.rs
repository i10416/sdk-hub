mod schema;
pub use schema::*;

use crate::PianoAPI;

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn activate_schedule_period(
        &self,
        req: &ActivatePeriodRequest,
    ) -> Result<(), crate::Error> {
        let _ = self
            .client
            .post(&format!(
                "{}/publisher/licensing/schedule/contract/periods/activate",
                self.endpoint,
            ))
            .form(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        todo!()
    }
}
