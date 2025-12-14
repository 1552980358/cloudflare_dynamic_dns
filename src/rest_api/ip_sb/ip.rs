use reqwest::{Client, StatusCode};
use serde::Deserialize;
use tokio::join;

use super::{
    error::Error,
    IpSBApi,
    Result
};

pub enum IP {
    V4(String),
    V6(String),
    Both { v4: String, v6: String }
}

impl IP {

    pub fn v4(&self) -> Option<String> {
        match self {
            Self::V4(v4) | Self::Both { v4, .. } => Some(v4.to_owned()),
            _ => None
        }
    }

    pub fn v6(&self) -> Option<String> {
        match self {
            Self::V6(v6) | Self::Both { v6, .. } => Some(v6.to_owned()),
            _ => None
        }
    }

}

mod url { 
    pub(super) const V4: &str = "https://api-ipv4.ip.sb/jsonip";
    pub(super) const V6: &str = "https://api-ipv6.ip.sb/jsonip";
}

impl IpSBApi {
    pub async fn get_ip(&self) -> Result<IP> {
        match join!(self.client.send_request_to(url::V4), self.client.send_request_to(url::V6)) {
            (Ok(v4), Ok(v6)) => Ok(IP::Both { v4, v6 }),
            (Ok(v4), Err(_)) => Ok(IP::V4(v4)),
            (Err(_), Ok(v6)) => Ok(IP::V6(v6)),
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

trait SendRequest {
    async fn send_request_to(&self, api_url: &str) -> Result<String>;
}

#[derive(Deserialize)]
struct ResponseBody {
    pub ip: String
}

impl SendRequest for Client {
    async fn send_request_to(&self, api_url: &str) -> Result<String> {
        self.get(api_url).send().await
            .map_err(|error|
                match error.status() {
                    // When incorrect user input is entered,
                    // the server returns an HTTP 400 Error (Bad Request),
                    // along with a JSON-encoded error message.
                    Some(status_code) if status_code == StatusCode::BAD_REQUEST => Error::Server,
                    _ if error.is_connect() || error.is_request() || error.is_timeout() => Error::Network,
                    _ => Error::Unknown
                }
            )?
            .json::<ResponseBody>().await
            .map(|response_body| response_body.ip)
            .map_err(|error| if error.is_decode() { Error::DecodeResponse } else { Error::Network })
    }
}

#[cfg(test)]
mod test {
    use log::info;
    
    use super::{IP, IpSBApi};

    #[tokio::test]
    async fn test_ip() {
        match IpSBApi::new().get_ip().await {
            Ok(ip) => match ip {
                IP::V4(v4) => {
                    info!("IPv4={v4}");
                    println!("IPv4={v4}");
                }
                IP::V6(v6) => {
                    info!("IPv6={v6}");
                    println!("IPv6={v6}");
                }
                IP::Both { v4, v6 } => {
                    info!("IPv4={v4} IPv6={v6}");
                    println!("IPv4={v4} IPv6={v6}");
                }
            },
            Err(err) => panic!("{:?}", err)
        }
    }
}