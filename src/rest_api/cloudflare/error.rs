#[derive(Debug)]
pub enum Error {
    Network,
    Unauthorized,
    DecodeResponse,
    Unknown
}