pub mod error;
pub mod ip;

use super::IpSBApi;
use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

impl IpSBApi {
    pub fn new(total_timeout: u64, connect_timeout: u64, read_timeout: u64) -> Self {
        use reqwest::Client;
        use std::time::Duration;
        let client = Client::builder()
            .timeout(Duration::from_secs(total_timeout))
            .connect_timeout(Duration::from_secs(connect_timeout))
            .read_timeout(Duration::from_secs(read_timeout))
            .build()
            .unwrap_or_else(|error| 
                if error.is_body() { panic!("Error occurred when building reqwest client") }
                else { panic!("Unknown error occurred when building reqwest client") }
            );
        
        Self { client }
    }
}