pub mod publisher;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::{io::ErrorKind, marker::PhantomData, str::FromStr};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct PianoAPI {
    app_id: String,
    endpoint: String,
    pub(crate) client: reqwest::Client,
}

impl PianoAPI {
    pub fn from_env() -> Self {
        let endpoint = std::env::var("PIANO_ENDPOINT").expect("PIANO_ENDPOINT is not set");
        let app_id = std::env::var("PIANO_APP_ID").expect("PIANO_APP_ID is not set");
        let token = std::env::var("PIANO_API_TOKEN").expect("PIANO_API_TOKEN is not set");
        Self::new(&endpoint, &app_id, &token)
    }
    pub fn new(endpoint: &str, app_id: &str, token: &str) -> Self {
        let client = reqwest::Client::builder()
            .default_headers(Self::mk_header(token))
            .build()
            .expect("Unable to build internal client");
        Self {
            endpoint: endpoint.to_string(),
            client,
            app_id: app_id.to_string(),
        }
    }
    fn mk_header(token: &str) -> HeaderMap {
        HeaderMap::from_iter([(
            HeaderName::from_str("API_TOKEN").expect("Always succeed"),
            HeaderValue::from_str(token).expect("Invalid Token"),
        )])
    }
}

#[derive(Debug, Serialize)]
pub struct PianoRequest<T: Serialize> {
    aid: String,
    #[serde(flatten)]
    inner: T,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PianoResponse<T> {
    Succeed(T),
    Failure {
        code: i32,
        message: String,
        #[serde(default)]
        validation_errors: Option<ValidationErrors>,
        #[serde(skip)]
        _phantom_data: PhantomData<T>,
    },
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Empty {
    #[serde(deserialize_with = "piano_code_from_usize")]
    code: PianoCode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PianoCode {
    Successful,
}

#[derive(Debug, Serialize, Default)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PianoPaginated<T> {
    // current limit
    pub limit: usize,
    // current offset
    pub offset: usize,
    // total number in db
    pub total: usize,
    // hit count
    pub count: usize,
    #[serde(flatten)]
    pub value: T,
}

impl<T> PianoPaginated<T> {
    pub fn has_next(&self) -> bool {
        self.offset + self.count < self.total
    }
}

#[derive(Debug, Deserialize)]
pub struct ValidationErrors {
    pub message: String,
}

impl<T: Clone> PianoResponse<T> {
    pub fn value(self) -> Result<T, crate::Error> {
        match self {
            Self::Succeed(t) => Result::Ok(t),
            Self::Failure {
                code,
                message,
                validation_errors,
                ..
            } => {
                let kind = match code {
                    61027 => ErrorKind::AlreadyExists,
                    _ => ErrorKind::Other,
                };
                let error = match validation_errors {
                    Some(validation) => format!("{code}: {message}: {}", validation.message),
                    None => format!("{code}: {message}"),
                };
                Result::Err(Box::new(std::io::Error::new(kind, error)))
            }
        }
    }
    pub fn maybe_value(self) -> Result<Option<T>, crate::Error> {
        match self {
            Self::Succeed(t) => Result::Ok(Some(t)),
            Self::Failure { code, message, .. } => {
                match code {
                    // schedule not found
                    61011 => Ok(None),
                    // contract not found
                    61002 => Ok(None),
                    // contract domain not found
                    61028 => Ok(None),
                    // contract ip domain not found
                    61035 => Ok(None),
                    // contract user not found
                    61020 => Ok(None),
                    // user not found
                    2004 => Ok(None),
                    61027 => Result::Err(Box::new(std::io::Error::new(
                        ErrorKind::AlreadyExists,
                        format!("{code}: {message}"),
                    ))),
                    _ => Result::Err(Box::new(std::io::Error::new(
                        ErrorKind::Other,
                        format!("{code}: {message}"),
                    ))),
                }
            }
        }
    }
}

fn piano_code_from_usize<'de, D>(deserializer: D) -> Result<PianoCode, D::Error>
where
    D: Deserializer<'de>,
{
    match usize::deserialize(deserializer)? {
        0 | 200 => Ok(PianoCode::Successful),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Empty, PianoCode};

    #[test]
    fn sanity_check_piano_empty_response() {
        let data = serde_json::json!({
            "code": 200
        });
        let result = serde_json::from_value::<Empty>(data);
        assert_eq!(
            result.expect("OK"),
            Empty {
                code: PianoCode::Successful
            }
        )
    }
}
