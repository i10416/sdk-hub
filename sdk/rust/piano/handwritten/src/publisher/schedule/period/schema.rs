use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AddPeriodRequest {
    pub schedule_id: String,
    pub name: String,
    pub sell_date: i64,
    pub begin_date: i64,
    pub end_date: i64,
}

#[derive(Debug, Serialize)]
pub struct UpdatePeriodRequest<'a> {
    pub schedule_id: &'a str,
    pub period_id: &'a str,
    pub name: &'a str,
    pub sell_date: i64,
    pub begin_date: i64,
    pub end_date: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct PeriodResult {
    pub period: Period,
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct Period {
    name: String,
    period_id: String,
    sell_date: u32,
    begin_date: u32,
    end_date: u32,
    deleted: bool,
    create_date: u32,
    update_date: u32,
    is_sale_started: bool,
    is_active: bool,
}
impl Period {
    pub fn period_id(&self) -> &str {
        &self.period_id
    }
}
