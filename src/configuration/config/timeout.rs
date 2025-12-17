use serde::Deserialize;

/**
 * For default timeout, see [Hypertext Transfer Protocol (HTTP) Timeouts](https://www.ietf.org/archive/id/draft-thomson-hybi-http-timeout-00.html#rfc.section.1.1)
 **/
#[derive(Deserialize)]
pub struct Timeout {
    #[serde(default = "default::total")]
    total: u64,
    #[serde(default = "default::connect")]
    connect: u64,
    #[serde(default = "default::read")]
    read: u64
}

mod default {

    pub(super) fn total() -> u64 { 300 }

    pub(super) fn connect() -> u64 { 120 }

    pub(super) fn read() -> u64 { 30 }

}

impl Timeout {
    pub fn all(&self) -> (u64, u64, u64) { 
        (self.total, self.connect, self.read) 
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Self {
            total: default::total(),
            connect: default::connect(),
            read: default::read()
        }
    }
}