pub mod error;
pub mod cloudflare;
pub mod config;

mod argument;
mod proxied;

use error::Error;
use argument::Argument;
use cloudflare::{Cloudflare, GetCloudflare};
use config::{Config, GetConfig};
use proxied::GetProxied;

pub struct Configuration {
    pub cloudflare: Cloudflare,
    pub config: Config,
    pub proxied: Option<bool>
}

pub type Result<T> = std::result::Result<T, Error>;

impl Configuration {
    pub fn new() -> Result<Self> {
        let arguments = Argument::all();

        let cloudflare = arguments.get_cloudflare()?;
        let config = arguments.get_config()?;
        let proxied = arguments.get_proxied();

        let configuration = Self { cloudflare, config, proxied };
        Ok(configuration)
    }
}