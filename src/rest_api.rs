
pub mod cloudflare;
pub mod ip_sb;

use reqwest::Client;
use ip_sb::IpSBApi;

pub trait GetIpSBApi {
    fn ip_sb_api(&'_ self) -> IpSBApi<'_>;
}

pub struct CloudflareApi {
    client: Client,
    zone: String
}