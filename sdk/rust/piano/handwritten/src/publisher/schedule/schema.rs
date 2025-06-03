use crate::publisher::schedule::period::Period;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct ScheduleResult {
    pub schedule: Schedule,
}
/// A “Schedule” is a set of fixed-time access periods
/// used in association with a payment term.
#[allow(unused)]
#[derive(Debug, Deserialize, Clone)]
pub struct Schedule {
    aid: String,
    name: String,
    schedule_id: String,
    deleted: bool,
    create_date: u32,
    update_date: u32,
    periods: Vec<Period>,
}
impl Schedule {
    pub fn schedule_id(&self) -> &str {
        &self.schedule_id
    }
    pub fn periods(&self) -> &Vec<Period> {
        &self.periods
    }
}
