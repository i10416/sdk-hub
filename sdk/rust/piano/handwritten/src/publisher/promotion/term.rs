// Placeholder for promotion term endpoints
// TODO: Implement promotion term endpoints when needed

mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// List terms assigned to promotion
    ///
    /// Lists terms assigned to a promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Fterm~2Flist)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_promotion_terms(
        &self,
        params: &ListPromotionTermRequest<'_>,
    ) -> Result<PianoPaginated<PromotionTermListResult>, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/promotion/term/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<PromotionTermListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Add term to promotion
    ///
    /// Adds a term to a given promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fpromotion~2Fterm~2Fadd)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn add_promotion_term(
        &self,
        req: &AddPromotionTermRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(format!("{}/publisher/promotion/term/add", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// Delete term from promotion
    ///
    /// Deletes a term from a promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fpromotion~2Fterm~2Fdelete)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_promotion_term(
        &self,
        req: &DeletePromotionTermRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(format!("{}/publisher/promotion/term/delete", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }
}
