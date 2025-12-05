use reqwest::Client;

pub mod error;
mod ip;

use super::IpSBApi;
use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

impl IpSBApi {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
}