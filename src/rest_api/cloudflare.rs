pub mod error;
pub mod record;
mod verify_user_token;

use super::CloudflareApi;
use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

impl CloudflareApi {
    // noinspection SpellCheckingInspection
    pub fn new(token: &String, zone: &String, total_timeout: u64, connect_timeout: u64, read_timeout: u64) -> Self {
        use reqwest::header::HeaderValue;
        let Ok(authorization_value) = HeaderValue::from_str(&format!("Bearer {}", &token)) else {
            panic!("Error occurred when building authorization header value");
        };

        use reqwest::header::HeaderMap;
        let mut headers = HeaderMap::new();
        use reqwest::header::AUTHORIZATION;
        headers.insert(AUTHORIZATION, authorization_value);

        use reqwest::Client;
        use std::time::Duration;
        let client = Client::builder()
            .timeout(Duration::from_secs(total_timeout))
            .connect_timeout(Duration::from_secs(connect_timeout))
            .read_timeout(Duration::from_secs(read_timeout))
            .default_headers(headers)
            .build()
            .unwrap_or_else(|err|
                if err.is_body() { panic!("Error occurred when building reqwest client") }
                else { panic!("Unknown error occurred when building reqwest client") }
            );
        let zone = zone.clone();
        Self { client, zone }
    }
}