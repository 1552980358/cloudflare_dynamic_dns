use serde::Deserialize;

use super::{
    super::{
        error::Error,
        CloudflareApi,
        Result
    },
    handle_network_error::HandleReqwestError,
    Record
};

#[derive(Deserialize)]
struct ResponseBody {
    #[serde(rename = "result")]
    pub records: Vec<Record>,
    pub success: bool
}

impl CloudflareApi {
    pub async fn list_record(&self) -> Result<Vec<Record>> {
        self.client.get(self.list_record_url())
            .send().await
            .handle_reqwest_error()?
            .json::<ResponseBody>().await
            .map_err(|error|
                if error.is_body() || error.is_decode() { Error::DecodeResponse } else { Error::Unknown }
            )
            .and_then(|response_body|
                if response_body.success { Ok(response_body.records) } else { Err(Error::Server) }
            )
    }
}

trait ListRecordsURL {
    fn list_record_url(&self) -> String;
}

impl ListRecordsURL for CloudflareApi {
    fn list_record_url(&self) -> String {
        format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", &self.zone)
    }
}