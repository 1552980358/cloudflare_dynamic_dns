use std::{
    env,
    path::PathBuf
};

mod help;
mod cloudflare_config;
mod config;

use cloudflare_config::HandleCloudflareConfig;
use config::HandleConfig;

pub(super) enum Argument {

    Config(PathBuf),

    CloudflareConfig(PathBuf),

}

impl<'argument> Argument {

    pub(super) fn all() -> Vec<Self> {
        let mut vec = Vec::new();
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                help::args::LONG | help::args::SHORT | help::args::SYMBOL => {
                    help::print_message();
                }
                cloudflare_config::args::LONG | cloudflare_config::args::SHORT => {
                    args.handle_cloudflare_config(&mut vec);
                }
                config::args::LONG | config::args::SHORT => {
                    args.handle_config(&mut vec);
                }
                // TODO: To be implemented
                _ => {
                    // TODO: To be implemented
                }
            }
        }
        vec
    }

}