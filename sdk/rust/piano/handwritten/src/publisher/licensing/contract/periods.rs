mod schema;
pub use schema::*;

use crate::{PianoAPI, PianoResponse};

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn activate_contract_periods(
        &self,
        req: &ActivatePeriodRequest,
    ) -> Result<SchedulePeriod, crate::Error> {
        let result = self
            .client
            .post(format!(
                "{}/publisher/licensing/contract/periods/activate",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<SchedulePeriodResult>>()
            .await?
            .value()?;
        Ok(result.schedule_period)
    }
}
