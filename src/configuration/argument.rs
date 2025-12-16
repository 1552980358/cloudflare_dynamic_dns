use std::{
    path::PathBuf,
    env
};

mod help;
mod cloudflare_config;
mod config;
mod proxied;

pub(super) enum Argument {

    Proxied(bool),

    Config(PathBuf),

    CloudflareConfig(PathBuf),

}

impl<'argument> Argument {

    pub(super) fn all() -> Vec<Self> {
        let mut vec = Vec::new();
        let mut args = env::args().skip(1).peekable();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                proxied::args::LONG | proxied::args::SHORT => {
                    use proxied::HandleProxied;
                    args.handle_proxied(&mut vec);
                }
                help::args::LONG | help::args::SHORT | help::args::SYMBOL => {
                    help::print_message();
                }
                cloudflare_config::args::LONG | cloudflare_config::args::SHORT => {
                    use cloudflare_config::HandleCloudflareConfig;
                    args.handle_cloudflare_config(&mut vec);
                }
                config::args::LONG | config::args::SHORT => {
                    use config::HandleConfig;
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