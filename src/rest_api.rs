
pub mod cloudflare;
pub mod ip_sb;

use ip_sb::IpSBApi;
use cloudflare::CloudflareApi;

pub trait GetIpSBApi {
    fn ip_sb_api(&'_ self) -> IpSBApi<'_>;
}

pub trait GetCloudflareApi {
    fn cloudflare_api(&'_ self) -> CloudflareApi<'_>;
}