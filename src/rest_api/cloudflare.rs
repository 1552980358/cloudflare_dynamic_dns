use reqwest::{
    header::{
        HeaderMap,
        HeaderValue,
        AUTHORIZATION
    },
    Client
};

pub mod error;
mod record;
mod verify_user_token;

use super::CloudflareApi;
use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

impl CloudflareApi {
    // noinspection SpellCheckingInspection
    pub fn new(token: &String, zone: &String) -> Self {
        let mut headers = HeaderMap::new();
        let Ok(authorization_value) = HeaderValue::from_str(&format!("Bearer {}", &token)) else {
            panic!("Error occurred when building authorization header value");
        };
        if let None = headers.insert(AUTHORIZATION, authorization_value) {
            panic!("Error occurred when adding authorization header");
        }

        let client = Client::builder()
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