// Placeholder for promotion code endpoints
// TODO: Implement promotion code endpoints when needed

mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// List promo codes
    ///
    /// Lists the promo codes of a given promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Fcode~2Flist)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_promotion_codes(
        &self,
        params: &ListPromotionCodeRequest<'_>,
    ) -> Result<PianoPaginated<PromotionCodeListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/promotion/code/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<PromotionCodeListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Create promo code
    ///
    /// Creates a promo code in a given promotion of a given app.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Fcode~2Fcreate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_promotion_code(
        &self,
        req: &CreatePromotionCodeRequest<'_>,
    ) -> Result<PromotionCode, crate::Error> {
        let result = self
            .client
            .get(&format!(
                "{}/publisher/promotion/code/create",
                self.endpoint
            ))
            .query(&[("aid", &self.app_id)])
            .query(req)
            .send()
            .await?
            .json::<PianoResponse<PromotionCodeResult>>()
            .await?
            .value()?;
        Ok(result.promo_code)
    }

    /// Count promo codes
    ///
    /// Returns the number of promo codes in a promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Fcode~2Fcount)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn count_promotion_codes(
        &self,
        params: &CountPromotionCodeRequest<'_>,
    ) -> Result<i32, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/promotion/code/count", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PromotionCodeCountResult>>()
            .await?
            .value()?;
        Ok(result.count)
    }

    /// Delete promo code
    ///
    /// Deletes a promo code from a promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fpromotion~2Fcode~2Fdelete)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_promotion_code(
        &self,
        req: &DeletePromotionCodeRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!(
                "{}/publisher/promotion/code/delete",
                self.endpoint
            ))
            .form(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }
}
