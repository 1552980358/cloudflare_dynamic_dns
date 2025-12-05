use serde::Deserialize;

use super::{
    super::{
        error::Error,
        CloudflareApi,
        Result
    },
    handle_network_error::HandleReqwestError,
};

#[derive(Deserialize)]
struct ResponseBody {
    pub success: bool
}

impl CloudflareApi {
    pub async fn delete_record(&self, record: &String) -> Result<()> {
        self.client.delete(self.delete_record_url(record))
            .send().await
            .handle_reqwest_error()?
            .json::<ResponseBody>().await
            .map_err(|error|
                if error.is_body() || error.is_decode() { Error::DecodeResponse } else { Error::Unknown }
            )
            .and_then(|response_body|
                if response_body.success { Ok(()) } else { Err(Error::Server) }
            )
    }
}

trait DeleteRecordUrl {
    fn delete_record_url(&self, record: &String) -> String;
}

impl DeleteRecordUrl for CloudflareApi {
    fn delete_record_url(&self, record: &String) -> String {
        format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", self.zone, record)
    }
}
