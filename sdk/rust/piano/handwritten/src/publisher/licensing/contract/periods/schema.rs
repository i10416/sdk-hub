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
    period_id: String,
    name: String,
    sell_date: i64,
    begin_date: i64,
    end_date: i64,
    status: SchedulePeriodStatus,
}

impl SchedulePeriod {
    pub fn period_id(&self) -> &str {
        &self.period_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn sell_date(&self) -> i64 {
        self.sell_date
    }
    pub fn begin_date(&self) -> i64 {
        self.begin_date
    }
    pub fn end_date(&self) -> i64 {
        self.end_date
    }
    pub fn status(&self) -> &SchedulePeriodStatus {
        &self.status
    }
}
#[derive(Debug, Deserialize, Clone)]
pub enum SchedulePeriodStatus {
    #[serde(rename = "ACTIVATED")]
    Activated,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "ENDED")]
    Ended,
}
