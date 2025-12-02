use serde::Deserialize;

use super::{
    super::{
        error::Error,
        CloudflareApi,
        Result
    },
};

#[derive(Deserialize)]
struct ResponseBody {
    pub success: bool
}

impl CloudflareApi {
    pub async fn delete_record(&self, record: &String) -> Result<()> {
        self.client.delete(self.delete_record_url(record))
            .send().await
            .map_err(|error|
                match error.status() {
                    /****************************************************************
                     * Missing Authorization header 400
                     * ```
                     * {
                     *     "success": false,
                     *     "errors": [
                     *         {
                     *           "code": 9106,
                     *           "message": "Missing X-Auth-Key, X-Auth-Email or Authorization headers"
                     *         }
                     *     ]
                     * }
                     * ```
                     ****************************************************************/
                    Some(status_code) if status_code == 400 => Error::Internal,
                    /****************************************************************
                     * Invalid Authorization header 401
                     * ```
                     * {
                     *     "success": false,
                     *     "errors": [
                     *          {
                     *            "code": 10000,
                     *            "message": "Authentication error"
                     *          }
                     *     ]
                     * }
                     * ```
                     ****************************************************************/
                    Some(status_code) if status_code == 401 => Error::Unauthorized,
                    /****************************************************************
                     * Invalid zone 403
                     * ```
                     * {
                     *     "success": false,
                     *     "errors": [
                     *          {
                     *            "code": 10000,
                     *            "message": "Authentication error"
                     *          }
                     *     ]
                     * }
                     * ```
                     ****************************************************************/
                    Some(status_code) if status_code == 403 => Error::InvalidZone,
                    /****************************************************************
                     * Invalid zone 404
                     * ```
                     * {
                     *     "result": null,
                     *     "success": false,
                     *     "errors": [
                     *         {
                     *           "code": 81044,
                     *           "message": "Record does not exist."
                     *         }
                     *     ],
                     *     "messages": []
                     * }
                     * ```
                     ****************************************************************/
                    Some(status_code) if status_code == 404 => Error::InvalidRecord,
                    _ => Error::Unknown
                }
            )?
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
