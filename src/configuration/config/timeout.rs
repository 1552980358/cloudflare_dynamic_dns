use serde::Deserialize;

/**
 * For default timeout, see [Hypertext Transfer Protocol (HTTP) Timeouts](https://www.ietf.org/archive/id/draft-thomson-hybi-http-timeout-00.html#rfc.section.1.1)
 **/
#[derive(Deserialize, Default)]
pub struct Timeout {
    #[serde(default = "default::total")]
    pub total: u64,
    #[serde(default = "default::connection")]
    pub connection: u64,
    #[serde(default = "default::read")]
    pub read: u64
}

mod default {

    pub(super) fn total() -> u64 { 300 }

    pub(super) fn connection() -> u64 { 120 }

    pub(super) fn read() -> u64 { 30 }

}