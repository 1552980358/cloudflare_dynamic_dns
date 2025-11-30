pub mod error;

use reqwest::Client;

use super::GetCloudflareApi;
use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct CloudflareApi<'cloudflare_api> {
    client: &'cloudflare_api Client
}

impl CloudflareApi<'_> {
    // TODO: Placeholder CloudflareApi implementation declaration
}

impl GetCloudflareApi for Client {
    fn cloudflare_api(&'_ self) -> CloudflareApi<'_> {
        CloudflareApi { client: self }
    }
}