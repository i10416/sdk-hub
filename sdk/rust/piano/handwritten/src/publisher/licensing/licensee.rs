mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoRequest, PianoResponse};

impl PianoAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_licensees<'a>(
        &self,
        params: &ListLicenseeRequest<'a>,
    ) -> Result<PianoPaginated<ListLicenseeResult>, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/licensing/licensee/list",
                self.endpoint,
            ))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<ListLicenseeResult>>>()
            .await?
            .value()?;
        Ok(result)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_licensee(
        &self,
        name: &str,
        representatives: &[String],
        uids: &[String],
    ) -> Result<Licensee, crate::Error> {
        let representatives = representatives
            .iter()
            .map(|email| Representative::new(email))
            .collect::<Vec<_>>();
        let representatives = serde_json::to_string(&representatives).ok();
        let manager_uids = uids.to_vec().join(",");
        let req = CreateLicenseeRequest {
            app_id: &self.app_id,
            manager_uids: &manager_uids,
            name,
            representatives: representatives.as_deref(),
        };
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/licensee/create",
                self.endpoint,
            ))
            .form(&req)
            .send()
            .await?
            .json::<PianoResponse<LicenseeResult>>()
            .await?
            .value()?;
        Ok(result.licensee)
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_licensee(
        &self,
        licensee_id: &str,
        name: &str,
        representatives: &[String],
        uids: &[String],
    ) -> Result<Licensee, crate::Error> {
        let representatives = representatives
            .iter()
            .map(|email| Representative::new(email))
            .collect::<Vec<_>>();
        let representatives = serde_json::to_string(&representatives).ok();
        let manager_uids = uids.to_vec().join(",");
        let inner = UpdateLicenseeRequest {
            licensee_id,
            manager_uids: &manager_uids,
            name,
            representatives: representatives.as_deref(),
        };
        let result = self
            .client
            .post(&format!(
                "{}/publisher/licensing/licensee/create",
                self.endpoint,
            ))
            .form(&PianoRequest {
                aid: self.app_id.to_string(),
                inner,
            })
            .send()
            .await?
            .json::<PianoResponse<LicenseeResult>>()
            .await?
            .value()?;
        Ok(result.licensee)
    }
}
