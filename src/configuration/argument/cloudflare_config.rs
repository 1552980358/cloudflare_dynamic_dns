use std::{
    env::Args,
    iter::{
        Peekable,
        Skip
    },
    path::PathBuf
};

use super::Argument;

pub(super) mod args {

    pub const LONG: &'static str = "--cloudflare";
    pub const SHORT: &'static str = "-cf";

}

pub(super) trait HandleCloudflareConfig {
    fn handle_cloudflare_config(&mut self, vec: &mut Vec<Argument>);
}

impl HandleCloudflareConfig for Peekable<Skip<Args>> {
    fn handle_cloudflare_config(&mut self, vec: &mut Vec<Argument>) {
        let Some(path) = self.next() else {
            panic!("Missing argument <path> to cloudflare json configuration file");
        };
        let path_buf = PathBuf::from(&path);
        if !path_buf.exists() || !path_buf.is_file() {
            panic!("Specified cloudflare json configuration file ({path}) does not exist");
        }
        vec.push(Argument::CloudflareConfig(path_buf))
    }
}
