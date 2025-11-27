use std::{
    fs::read_to_string,
    path::PathBuf
};
use log::info;
use serde::Deserialize;

pub mod domain_name;

use super::{
    Argument,
    Result,
    error::Error
};
use domain_name::DomainName;

#[derive(Deserialize)]
pub struct Cloudflare {
    pub token: String,
    pub zone: String,
    pub domain_names: Vec<DomainName>,
}

impl<'cloudflare> Cloudflare {
    const DEFAULT_PATH: &'cloudflare str = "/usr/local/etc/cloudflare_dynamic_dns/cloudflare.conf.json";
}

pub(super) trait GetCloudflare {
    fn get_cloudflare(&self) -> Result<Cloudflare>;
}

impl GetCloudflare for Vec<Argument> {
    fn get_cloudflare(&self) -> Result<Cloudflare> {
        self.iter()
            .find_map(|argument|
                if let Argument::CloudflareConfig(path_buf) = argument { Some(path_buf.to_owned()) }
                else { None }
            )
            .unwrap_or_else(|| {
                info!(
                    target: "configuration.cloudflare",
                    r#"Cloudflare json configuration file not specified: Use default path "{}" instead."#,
                    Cloudflare::DEFAULT_PATH
                );
                PathBuf::from(Cloudflare::DEFAULT_PATH)
            })
            .read_json_file()?
            .deserialize()
    }
}

trait ReadFile {
    fn read_json_file(&self) -> Result<String>;
}

impl ReadFile for PathBuf {
    fn read_json_file(&self) -> Result<String> {
        read_to_string(self).map_err(|_| Error::CloudflareImportFail)
    }
}

trait DeserializeCloudflare {
    fn deserialize(&self) -> Result<Cloudflare>;
}

impl DeserializeCloudflare for String {
    fn deserialize(&self) -> Result<Cloudflare> {
        serde_json::from_str::<Cloudflare>(self).map_err(|_| Error::CloudflareImportFail)
    }
}