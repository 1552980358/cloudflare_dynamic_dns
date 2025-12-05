use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::{
    super::{
        error::Error,
        CloudflareApi,
        Result
    },
    Record
};

#[derive(Serialize)]
#[serde(untagged)]
enum RequestBody {
    Value {
        #[serde(rename = "content")]
        value: String
    },
    Proxied {
        #[serde(rename = "proxied")]
        is_proxied: bool
    }
}

#[derive(Deserialize)]
struct ResponseBody {
    #[serde(rename = "result")]
    pub record: Option<Record>
}

impl CloudflareApi {

    async fn update_record(&self, record: &String, request_body: RequestBody) -> Result<Record> {
        self.client.patch(self.update_record_url(record))
            .json(&request_body)
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
                    _ if error.is_request() || error.is_connect() || error.is_timeout() || error.is_status() => Error::Network,
                    _ => Error::Unknown
                }
            )?
            .json::<ResponseBody>().await
            .map_err(|error|
                if error.is_body() || error.is_decode() { Error::DecodeResponse } else { Error::Unknown }
            )
            .and_then(|response_body|
                if let Some(record) = response_body.record { Ok(record) } else { Err(Error::Server) }
            )
    }

    pub async fn update_record_value(&self, record: &String, value: &String) -> Result<Record> {
        self.update_record(record, RequestBody::from_value(value)).await
    }

    pub async fn update_record_proxied(&self, record: &String, is_proxied: bool) -> Result<Record> {
        self.update_record(record, RequestBody::from_is_proxied(is_proxied)).await
    }

}

impl RequestBody {

    pub(super) fn from_value(value: &String) -> Self {
        Self::Value { value: value.clone() }
    }

    pub(super) fn from_is_proxied(is_proxied: bool) -> Self {
        Self::Proxied { is_proxied }
    }

}

trait UpdateRecordUrl {
    fn update_record_url(&self, record: &String) -> String;
}

impl UpdateRecordUrl for CloudflareApi {
    fn update_record_url(&self, record: &String) -> String {
        format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", self.zone, record)
    }
}


#[cfg(test)]
mod test {
    use super::RequestBody;

    #[test]
    pub fn serialize() {
        assert_eq!(
            serde_json::to_string(&RequestBody::from_value(&"192.168.1.110".to_string())).unwrap(),
            r#"{"content":"192.168.1.110"}"#
        );
        assert_eq!(
            serde_json::to_string(&RequestBody::from_is_proxied(true)).unwrap(),
            r#"{"proxied":true}"#
        );
    }

}