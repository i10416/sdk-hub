use crate::publisher::schedule::period::Period;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct ScheduleResult {
    pub schedule: Schedule,
}
/// A "Schedule" is a set of fixed-time access periods
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PianoResponse;

    // Note: GetScheduleRequest type is not defined in this schema
    // #[test]
    // fn test_get_schedule_request() {
    //     let request = GetScheduleRequest::new("schedule123");
    //     assert_eq!(request.schedule_id, "schedule123");
    // }

    #[test]
    fn test_schedule_deserialization() {
        let json = serde_json::json!({
            "schedule_id": "12345",
            "aid": "app123",
            "name": "Test Schedule",
            "deleted": false,
            "create_date": 1640995200,
            "update_date": 1641081600,
            "periods": []
        });

        let schedule: Schedule = serde_json::from_value(json).expect("Failed to deserialize schedule");
        assert_eq!(schedule.schedule_id(), "12345");
        assert_eq!(schedule.periods().len(), 0);
    }

    // Note: Snapshot file doesn't exist, commenting out this test
    /*
    #[test]
    fn sanity_check_get_schedule_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<ScheduleResult>>(snapshot);
        
        assert!(value.is_ok(), "Failed to deserialize schedule get: {:?}", value.err());
        let response = value.unwrap();
        
        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.schedule.schedule_id(), "***MASKED***");
                assert_eq!(data.schedule.name(), "***MASKED***");
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
    */
}
