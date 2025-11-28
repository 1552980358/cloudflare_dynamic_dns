pub mod error;
mod argument;
mod cloudflare;
mod config;
mod proxied;

use error::Error;
use proxied::GetProxied;

pub struct Configuration {
    // TODO: Placeholder struct declaration
}

pub type Result<T> = std::result::Result<T, Error>;