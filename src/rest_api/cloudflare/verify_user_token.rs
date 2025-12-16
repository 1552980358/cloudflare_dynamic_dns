use serde::Deserialize;

use super::{CloudflareApi, Result};

const URL: &str = "https://api.cloudflare.com/client/v4/user/tokens/verify";

#[derive(Deserialize)]
struct ResponseBody {
    pub success: bool
}

impl CloudflareApi {
    pub async fn verify_user_token(&self) -> Result<()> {
        self.client.get(URL)
            .send().await
            .map_err(|error| {
                use reqwest::StatusCode;
                use super::error::Error;
                match error.status() {
                    Some(status_code) if status_code == StatusCode::BAD_REQUEST => Error::Unauthorized,
                    _ if error.is_request() || error.is_connect() || error.is_timeout() || error.is_status() => Error::Network,
                    _ => Error::Unknown
                }
            })?
            .json::<ResponseBody>().await
            .map_err(|error| {
                use super::error::Error;
                if error.is_decode() { Error::DecodeResponse } else { Error::Network }
            })
            .and_then(|response_body|
                if response_body.success { 
                    Ok(()) 
                } 
                else {
                    use super::error::Error;
                    Err(Error::Unauthorized) 
                }
            )
    }
}