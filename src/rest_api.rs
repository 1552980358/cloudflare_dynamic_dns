
pub mod cloudflare;
pub mod ip_sb;

use ip_sb::IpSBApi;

pub trait GetIpSBApi {
    fn ip_sb_api(&'_ self) -> IpSBApi<'_>;
}