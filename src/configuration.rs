pub mod error;
use error::Error;

pub struct Configuration {
    // TODO: Placeholder struct declaration
}

pub type Result<T> = std::result::Result<T, Error>;
