pub mod error;
pub mod cloudflare;
pub mod config;

mod argument;
mod proxied;

use error::Error;
use cloudflare::Cloudflare;
use config::Config;

pub struct Configuration {
    pub cloudflare: Cloudflare,
    pub config: Config,
    pub proxied: Option<bool>
}

pub type Result<T> = std::result::Result<T, Error>;

impl Configuration {
    pub fn new() -> Result<Self> {
        use argument::Argument;
        let arguments = Argument::all();

        use cloudflare::GetCloudflare;
        use config::GetConfig;
        use proxied::GetProxied;
        let (cloudflare, config, proxied) = (
            arguments.get_cloudflare()?, arguments.get_config()?, arguments.get_proxied()
        );

        let configuration = Self { cloudflare, config, proxied };
        Ok(configuration)
    }
}