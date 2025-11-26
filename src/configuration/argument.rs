use std::env;

mod help;

pub(super) enum Argument {

    ConfigFile(String),

    CloudflareConfigFile(String),

}

impl<'argument> Argument {

    pub(super) fn from_env() -> Vec<Self> {
        let mut vec = Vec::new();
        let mut env_args_iter = env::args().skip(1);
        while let Some(arg) = env_args_iter.next() {
            match arg.as_str() {
                help::args::SHORT | help::args::LONG | help::args::SYMBOL => {
                    help::print_message();
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