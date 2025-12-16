use reqwest::Client;

pub mod cloudflare;
pub mod ip_sb;

pub struct IpSBApi {
    client: Client
}

pub struct CloudflareApi {
    client: Client,
    zone: String
}