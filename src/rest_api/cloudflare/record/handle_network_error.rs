// noinspection SpellCheckingInspection
use reqwest::{Error as ReqwestError, Response, StatusCode};

use super::super::error::Error;

// noinspection SpellCheckingInspection
pub(super) trait HandleReqwestError {
    fn handle_reqwest_error(self) -> Result<Response, Error>;
}

impl HandleReqwestError for Result<Response, ReqwestError> {
    // noinspection SpellCheckingInspection
    fn handle_reqwest_error(self) -> Result<Response, Error> {
        self.map_err(|error|
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
                Some(status_code) if status_code == StatusCode::NOT_FOUND => Error::InvalidRecord,
                _ if error.is_request() || error.is_connect() || error.is_timeout() || error.is_status() => Error::Network,
                _ => Error::Unknown
            }
        )
    }
}