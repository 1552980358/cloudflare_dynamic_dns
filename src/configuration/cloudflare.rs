use serde::Deserialize;

pub mod domain_name;

use super::{
    argument::Argument,
    error::Error,
    Result
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
        let path_buf = self.iter()
            .find_map(|argument|
                if let Argument::CloudflareConfig(path_buf) = argument { Some(path_buf.to_owned()) }
                else { None }
            )
            .unwrap_or_else(|| {
                use log::info;
                info!(
                    target: "configuration.cloudflare",
                    r#"Cloudflare json configuration file not specified: Use default path "{}" instead."#,
                    Cloudflare::DEFAULT_PATH
                );
                use std::path::PathBuf;
                PathBuf::from(Cloudflare::DEFAULT_PATH)
            });

        use std::fs::read_to_string;
        let cloudflare_config_json_str = read_to_string(&path_buf)
            .map_err(|_| Error::CloudflareImportFail(path_buf.to_string_lossy().to_string()))?;

        serde_json::from_str(&cloudflare_config_json_str)
            .map_err(|_| Error::CloudflareImportFail(path_buf.to_string_lossy().to_string()))
    }
}