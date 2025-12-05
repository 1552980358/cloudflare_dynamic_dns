
pub mod cloudflare;
pub mod ip_sb;

use reqwest::Client;

pub struct IpSBApi {
    client: Client
}

pub struct CloudflareApi {
    client: Client,
    zone: String
}