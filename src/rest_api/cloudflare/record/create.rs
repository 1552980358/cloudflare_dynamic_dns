use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::{
    super::{
        error::Error,
        CloudflareApi,
        Result
    },
    Record,
    RecordType
};

#[derive(Serialize)]
struct RequestBody {
    #[serde(rename = "name")]
    domain_name: String,
    #[serde(rename = "content")]
    value: String,
    #[serde(rename = "type")]
    record_type: RecordType,
    #[serde(rename = "ttl")]
    time_to_live: u16,
    #[serde(rename = "proxied")]
    is_proxied: bool
}

#[derive(Deserialize)]
struct ResponseBody {
    result: Record,
    success: bool
}

impl CloudflareApi {
    pub async fn create_record(
        &self, domain_name: &String, value: &String, record_type: RecordType, time_to_live: u16, is_proxied: bool
    ) -> Result<Record> {
        self.client.post(self.create_record_url())
            .json(&RequestBody::new(domain_name, value, record_type, time_to_live, is_proxied))
            .send().await
            .map_err(|error|
                match error.status() {
                    /****************************************************************
                     * Missing Authorization header 400
                     * ```
                     * {
                     * "success": false,
                     *     "errors": [
                     *         {
                     *             "code": 9106,
                     *             "message": "Missing X-Auth-Key, X-Auth-Email or Authorization headers"
                     *         }
                     *     ]
                     * }
                     * ```
                     * Missing request body 400
                     * ````
                     * {
                     *     "result": null,
                     *     "success": false,
                     *     "errors": [
                     *         {
                     *             "code": 9207,
                     *             "message": "Request body is invalid."
                     *         }
                     *     ],
                     *     "messages": []
                     * }
                     * ```
                     ****************************************************************/
                    Some(status_code) if status_code == StatusCode::BAD_REQUEST => Error::Internal,
                    /****************************************************************
                     * Invalid Authorization header 401
                     * ```
                     * {
                     *     "success": false,
                     *     "errors": [
                     *         {
                     *             "code": 10000,
                     *             "message": "Authentication error"
                     *         }
                     *     ]
                     * }
                     * ```
                     ****************************************************************/
                    Some(status_code) if status_code == StatusCode::UNAUTHORIZED => Error::Unauthorized,
                    /****************************************************************
                     * Invalid zone id 403
                     * ```
                     * {
                     *     "success": false,
                     *     "errors": [
                     *         {
                     *             "code": 10000,
                     *             "message": "Authentication error"
                     *         }
                     *     ]
                     * }
                     * ```
                     ****************************************************************/
                    Some(status_code) if status_code == StatusCode::FORBIDDEN => Error::InvalidZone,
                    _ => Error::Unknown
                }
            )?
            .json::<ResponseBody>().await
            .map_err(|error|
                if error.is_body() || error.is_decode() { Error::DecodeResponse } else { Error::Unknown }
            )
            .and_then(|response_body|
                if response_body.success { Ok(response_body.result) } else { Err(Error::Server) }
            )
    }
}

impl RequestBody {
    fn new(domain_name: &String, value: &String, record_type: RecordType, time_to_live: u16, is_proxied: bool) -> Self {
        Self {
            domain_name: domain_name.clone(),
            value: value.clone(),
            record_type,
            time_to_live,
            is_proxied
        }
    }
}

trait CreateRecordUrl {
    fn create_record_url(&self) -> String;
}

impl CreateRecordUrl for CloudflareApi {
    fn create_record_url(&self) -> String {
        format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", self.zone)
    }
}