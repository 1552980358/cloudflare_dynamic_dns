use std::fs::read_to_string;
use std::path::PathBuf;
use serde::Deserialize;

use super::{
    argument::Argument,
    error::Error,
    Result
};

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(rename = "unavailable-hide", default = "default::unavailable_hide")]
    pub unavailable_hide: bool
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
        self.iter()
            .find_map(|argument| if let Argument::Config(path_buf) = argument { Some(path_buf) } else { None })
            .map(|path_buf| path_buf.read_json_file().and_then(String::deserialize_config))
            .unwrap_or_else(|| Ok(Config::default()))
    }
}

trait ReadJsonFile {
    fn read_json_file(&self) -> Result<String>;
}

impl ReadJsonFile for PathBuf {
    fn read_json_file(&self) -> Result<String> {
        read_to_string(self).map_err(|_| Error::ConfigImportFail)
    }
}

trait DeserializeConfig {
    fn deserialize_config(self) -> Result<Config>;
}

impl DeserializeConfig for String {
    fn deserialize_config(self) -> Result<Config> {
        serde_json::from_str::<Config>(&self).map_err(|_| Error::ConfigImportFail)
    }
}