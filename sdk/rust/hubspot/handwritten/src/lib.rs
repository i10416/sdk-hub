use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub mod crm;

#[derive(Debug, Clone)]
pub struct HubAPI {
    pub(crate) client: reqwest::Client,
}

impl HubAPI {
    pub fn new(token: &str) -> Self {
        let client = reqwest::Client::builder()
            .default_headers(Self::mk_header(token))
            .build()
            .expect("Unable to build internal client");
        Self { client }
    }
    fn mk_header(token: &str) -> HeaderMap {
        HeaderMap::from_iter([(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&format!("Bearer {token}")).expect("Invalid Token"),
        )])
    }
}
