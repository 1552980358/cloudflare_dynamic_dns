pub mod error;
mod argument;

use error::Error;

pub struct Configuration {
    // TODO: Placeholder struct declaration
}

pub type Result<T> = std::result::Result<T, Error>;