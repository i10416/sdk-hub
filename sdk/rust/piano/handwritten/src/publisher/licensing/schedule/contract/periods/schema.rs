use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActivatePeriodRequest {
    pub schedule_id: String,
}
