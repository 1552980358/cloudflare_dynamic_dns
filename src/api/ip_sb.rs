use reqwest::Client;
use serde::Deserialize;
use tokio::join;

pub mod error;

use super::AccessIpSB;
use error::Error;

pub enum IpSB {
    V4(String),
    V6(String),
    Both { v4: String, v6: String }
}

type Result<T> = std::result::Result<T, Error>;

impl<'ip_sb> IpSB {

    // We do request a json formatted result, so that we do not need to handle suffix new line from HTTP
    const API_URL_V4: &'ip_sb str = "https://api-ipv4.ip.sb/jsonip";
    const API_URL_V6: &'ip_sb str = "https://api-ipv6.ip.sb/jsonip";

    pub async fn request(client: &Client) -> Result<Self> {
        match join!(client.api_get(Self::API_URL_V4), client.api_get(Self::API_URL_V6)) {
            (Ok(v4), Ok(v6)) => Ok(Self::Both { v4, v6 }),
            (Ok(v4), Err(_)) => Ok(Self::V4(v4)),
            (Err(_), Ok(v6)) => Ok(Self::V6(v6)),
            (Err(v4_err), Err(v6_err)) => {
                // Any single network error will cause to network error returned
                if matches!(v4_err, Error::Network) || matches!(v6_err, Error::Network) { Err(Error::Network) }
                // Any single server error will cause to server error returned
                else if matches!(v4_err, Error::Server) || matches!(v6_err, Error::Server) { Err(Error::Server) }
                else { Err(Error::Unknown) }
            }
        }
    }

}

trait ApiGet {
    async fn api_get(&self, api_url: &str) -> Result<String>;
}

#[derive(Deserialize)]
struct ResponseBody {
    pub ip: String
}

impl ApiGet for Client {
    async fn api_get(&self, api_url: &str) -> Result<String> {
        self.get(api_url).send().await
            .map_err(|err|
                match err.status() {
                    // When incorrect user input is entered,
                    // the server returns an HTTP 400 Error (Bad Request),
                    // along with a JSON-encoded error message.
                    Some(status_code) if status_code == 400 => Error::Server,
                    _ if err.is_connect() || err.is_request() || err.is_timeout() => Error::Network,
                    _ => Error::Unknown
                }
            )?
            .json::<ResponseBody>().await
            .map(|response_body| response_body.ip)
            .map_err(|_| Error::Server)
    }
}

impl AccessIpSB for Client {
    async fn access_ip_sb(&self) -> IpSB {
        IpSB::request(self).await
            .unwrap_or_else(|err| {
                match err {
                    Error::Network => panic!("IP.SB: Network error occurred when requesting ip addresses"),
                    Error::Server => panic!("IP.SB: Server error occurred when requesting ip addresses"),
                    Error::Unknown => panic!("IP.SB: Unknown error occurred when requesting ip addresses"),
                }
            })
    }
}

#[cfg(test)]
mod test {
    use log::trace;
    use reqwest::Client;
    use super::{AccessIpSB, IpSB};

    #[tokio::test]
    async fn get_ip_sb() {
        match Client::new().access_ip_sb().await {
            IpSB::V4(v4) => {
                trace!(target: "api.ip-sb","IPv4={v4}");
                println!("IPv4={v4}");
            }
            IpSB::V6(v6) => {
                trace!(target: "api.ip-sb","IPv6={v6}");
                println!("IPv6={v6}");
            }
            IpSB::Both { v4, v6 } => {
                trace!(target: "api.ip-sb","IPv4={v4} IPv6={v6}");
                println!("IPv4={v4} IPv6={v6}");
            }
        }
    }

}