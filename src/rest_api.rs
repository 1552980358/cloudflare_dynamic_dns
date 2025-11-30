pub mod cloudflare;
pub mod ip_sb;

use ip_sb::IpSB;

pub trait AccessIpSB {
    async fn access_ip_sb(&self) -> IpSB;
}
