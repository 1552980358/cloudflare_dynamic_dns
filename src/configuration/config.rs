use serde::Deserialize;

use super::{
    argument::Argument,
    Result
};

mod timeout;
use timeout::Timeout;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(rename = "unavailable-hide", default = "default::unavailable_hide")]
    pub unavailable_hide: bool,
    #[serde(rename = "ip.sb-timeout", default)]
    pub ip_sb_timeout: Timeout,
    #[serde(rename = "cloudflare-timeout", default)]
    pub cloudflare_timeout: Timeout
    // TODO: Non-finalized declaration, leave for future needed
}

mod default {
    pub(super) fn unavailable_hide() -> bool { true }
}

pub(super) trait GetConfig {
    fn get_config(&self) -> Result<Config>;
}

impl GetConfig for Vec<Argument> {
    fn get_config(&self) -> Result<Config> {
        let path_buf = self.iter()
            .find_map(|argument| if let Argument::Config(path_buf) = argument { Some(path_buf) } else { None });

        if let Some(path_buf) = path_buf {
            use std::fs::read_to_string;
            let config_json_str = read_to_string(&path_buf).map_err(|_| {
                use super::error::Error;
                Error::ConfigImportFail(path_buf.to_string_lossy().to_string())
            })?;

            serde_json::from_str(&config_json_str)
                .map_err(|_| {
                    use super::error::Error;
                    Error::ConfigImportFail(path_buf.to_string_lossy().to_string())
                })
        }
        else {
            Ok(Config::default())
        }
    }
}