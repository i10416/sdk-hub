pub mod code;
mod schema;
pub mod term;
pub use self::schema::*;

use crate::{PianoAPI, PianoPaginated, PianoResponse};

impl PianoAPI {
    /// Get a promotion by ID
    ///
    /// Gets a promotion selected by ID.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Fget)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_promotion(
        &self,
        promotion_id: &str,
    ) -> Result<Option<Promotion>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/promotion/get", self.endpoint))
            .query(&[
                ("aid", &self.app_id),
                ("promotion_id", &promotion_id.to_string()),
            ])
            .send()
            .await?
            .json::<PianoResponse<PromotionResult>>()
            .await?
            .maybe_value()?;
        Ok(result.map(|r| r.promotion))
    }

    /// Create a new promotion
    ///
    /// Creates a promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fpromotion~2Fcreate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_promotion(
        &self,
        req: &CreatePromotionRequest<'_>,
    ) -> Result<Promotion, crate::Error> {
        let result = self
            .client
            .post(&format!("{}/publisher/promotion/create", self.endpoint))
            .form(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<PromotionResult>>()
            .await?
            .value()?;
        Ok(result.promotion)
    }

    /// Update an existing promotion
    ///
    /// Updates a promotion and returns its Promotion object.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fpromotion~2Fupdate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_promotion(
        &self,
        req: &UpdatePromotionRequest<'_>,
    ) -> Result<Promotion, crate::Error> {
        let result = self
            .client
            .post(&format!("{}/publisher/promotion/update", self.endpoint))
            .form(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<PromotionResult>>()
            .await?
            .value()?;
        Ok(result.promotion)
    }

    /// Delete a promotion
    ///
    /// Deletes a promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fpromotion~2Fdelete)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_promotion(
        &self,
        req: &DeletePromotionRequest<'_>,
    ) -> Result<(), crate::Error> {
        self.client
            .post(&format!("{}/publisher/promotion/delete", self.endpoint))
            .form(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?;
        Ok(())
    }

    /// List promotions
    ///
    /// Lists promotions with pagination and filtering options.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Flist)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_promotions(
        &self,
        params: &ListPromotionRequest,
    ) -> Result<PianoPaginated<PromotionListResult>, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/promotion/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PianoPaginated<PromotionListResult>>>()
            .await?
            .value()?;
        Ok(result)
    }

    /// Count promotions
    ///
    /// Returns the count of promotions for a given app.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Fcount)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn count_promotions(
        &self,
        params: &CountPromotionRequest,
    ) -> Result<i32, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/promotion/count", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<PromotionCountResult>>()
            .await?
            .value()?;
        Ok(result.count)
    }

    /// Check if promotion exists
    ///
    /// Checks if a promotion with the given name exists.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=get~2F~2Fpublisher~2Fpromotion~2Fexists)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn promotion_exists(&self, name: &str) -> Result<bool, crate::Error> {
        let result = self
            .client
            .get(&format!("{}/publisher/promotion/exists", self.endpoint))
            .query(&[("aid", &self.app_id), ("name", &name.to_string())])
            .send()
            .await?
            .json::<PianoResponse<PromotionExistsResult>>()
            .await?
            .value()?;
        Ok(result.exists)
    }

    /// Generate promotion codes
    ///
    /// Generates promotion codes for a given promotion.
    ///
    /// See: [Piano API Documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fpromotion~2Fgenerate)
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn generate_promotion_codes(
        &self,
        req: &GeneratePromotionRequest<'_>,
    ) -> Result<GeneratePromotionResult, crate::Error> {
        let result = self
            .client
            .post(&format!("{}/publisher/promotion/generate", self.endpoint))
            .form(&[("aid", &self.app_id)])
            .form(req)
            .send()
            .await?
            .json::<PianoResponse<GeneratePromotionResult>>()
            .await?
            .value()?;
        Ok(result)
    }
}
