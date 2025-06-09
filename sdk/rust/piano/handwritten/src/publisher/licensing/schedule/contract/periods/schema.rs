use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActivatePeriodRequest<'a> {
    pub schedule_id: &'a str,
}

impl<'a> ActivatePeriodRequest<'a> {
    pub fn new(schedule_id: &'a str) -> Self {
        Self {
            schedule_id: schedule_id,
        }
    }
}
