use reqwest::Client;
use serde::Deserialize;
use tokio::join;

use super::{
    error::Error,
    Result
};

pub enum IP {
    V4(String),
    V6(String),
    Both { v4: String, v6: String }
}

mod url { 
    pub(super) const V4: &str = "https://api-ipv4.ip.sb/jsonip";
    pub(super) const V6: &str = "https://api-ipv6.ip.sb/jsonip";
}

pub(super) async fn get(client: &Client) -> Result<IP> {
    match join!(client.api_get(url::V4), client.api_get(url::V6)) {
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

trait ApiGetRequest {
    async fn api_get(&self, api_url: &str) -> Result<String>;
}

#[derive(Deserialize)]
struct ResponseBody {
    pub ip: String
}

impl ApiGetRequest for Client {
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

#[cfg(test)]
mod test {
    use reqwest::Client;
    use super::{get, IP};

    #[tokio::test]
    async fn test_ip() {
        match get(&Client::new()).await { 
            Ok(ip) => match ip {
                IP::V4(v4) => {}
                IP::V6(v6) => {}
                IP::Both { v4, v6 } => {
                    
                }
            },
            Err(err) => panic!("{:?}", err)
        }
    }
}