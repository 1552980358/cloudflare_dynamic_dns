use reqwest::Client;

pub mod error;
mod ip;

pub use ip::IP;
use super::GetIpSBApi;
use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct IpSBApi<'ip_sb_api> {
    client: &'ip_sb_api Client
}

impl GetIpSBApi for Client {
    fn ip_sb_api(&'_ self) -> IpSBApi<'_> {
        IpSBApi { client: self }
    }
}