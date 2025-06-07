use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActivatePeriodRequest<'a> {
    pub schedule_id: &'a str,
}
