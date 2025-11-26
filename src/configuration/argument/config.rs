use std::{
    env::Args,
    iter::Skip,
    path::PathBuf
};

use super::Argument;

pub(super) mod args {

    pub const LONG: &'static str = "--config";
    pub const SHORT: &'static str = "-c";

}

pub(super) trait HandleConfig {
    fn handle_config(&mut self, vec: &mut Vec<Argument>);
}

impl HandleConfig for Skip<Args>  {
    fn handle_config(&mut self, vec: &mut Vec<Argument>) {
        let Some(path) = self.next() else {
            panic!("Missing argument <path> to configuration file");
        };
        let path_buf = PathBuf::from(&path);
        if !path_buf.exists() || !path_buf.is_file() {
            panic!("Configuration file path ({path}) does not exist");
        }
        vec.push(Argument::Config(path_buf))
    }
}