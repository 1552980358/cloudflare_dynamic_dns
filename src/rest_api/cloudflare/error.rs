#[derive(Debug)]
pub enum Error {
    Internal,
    Network,
    Unauthorized,
    InvalidZone,
    InvalidRecord,
    Server,
    DecodeResponse,
    Unknown
}