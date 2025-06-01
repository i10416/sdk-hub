use crate::crm::v4::objects::{DeleteNestedObjectRequest, GetNestedObjectRequest, List};

type Resource = ();

impl crate::HubAPI {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn create_deals_associations(&self) {}
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_deals_associations(
        &self,
        deal_id: &str,
    ) -> Result<List<Resource>, crate::crm::v3::Error> {
        self.v4_list_nested_objects("deals", deal_id, "associations", Default::default())
            .await
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_deals_associations(
        &self,
        deal_id: &str,
        id: &str,
    ) -> Result<Resource, crate::crm::v3::Error> {
        let _ = self
            .v4_get_nested_object::<serde_json::Value>(GetNestedObjectRequest {
                parent_name: "deals".to_string(),
                parent_id: deal_id.to_string(),
                name: "associations".to_string(),
                id: id.to_string(),
            })
            .await?;
        Ok(())
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn update_deals_associations(&self) {}

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]

    pub async fn delete_deals_associations(
        &self,
        deal_id: &str,
        id: &str,
    ) -> Result<(), crate::crm::v3::Error> {
        let _ = self
            .v4_delete_nested_object(DeleteNestedObjectRequest {
                parent_name: "deals".to_string(),
                parent_id: deal_id.to_string(),
                name: "associations".to_string(),
                id: id.to_string(),
            })
            .await?;
        Ok(())
    }
}
