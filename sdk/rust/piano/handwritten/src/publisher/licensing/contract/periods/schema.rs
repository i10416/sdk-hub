use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ActivatePeriodRequest {
    contract_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct SchedulePeriodResult {
    pub schedule_period: SchedulePeriod,
}

#[derive(Debug, Deserialize, Clone)]

pub struct SchedulePeriod {
    pub period_id: String,
    pub name: String,
    pub sell_date: u32,
    pub begin_date: u32,
    pub end_date: u32,
    pub status: String,
}
