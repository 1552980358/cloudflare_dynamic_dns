use std::{
    env::Args,
    iter::{
        Peekable,
        Skip
    }
};
use serde::Deserialize;

use super::Argument;

pub(super) mod args {

    pub const LONG: &'static str = "--proxied";
    pub const SHORT: &'static str = "-p";

}

pub(super) trait HandleProxied {
    fn handle_proxied(&mut self, vec: &mut Vec<Argument>);
}

#[derive(Deserialize, Debug)]
enum Status {
    #[serde(rename = "on", alias = "1", alias = "true", alias = "enable")]
    On,
    #[serde(rename = "off", alias = "0", alias = "false", alias = "disable")]
    Off
}

impl HandleProxied for Peekable<Skip<Args>> {
    fn handle_proxied(&mut self, vec: &mut Vec<Argument>) {
        let is_proxied = self.peek()
            .and_then(String::deserialize_status)
            .map_or(true, |status| {
                // Do move iter into next, as it was deserialized successfully
                self.next();
                status.into()
            });
        vec.push(Argument::Proxied(is_proxied));
    }
}

impl Into<bool> for Status {
    fn into(self) -> bool {
        match self {
            Status::On => { true }
            Status::Off => { false }
        }
    }
}

trait DeserializeStatus {
    fn deserialize_status(&self) -> Option<Status>;
}

impl DeserializeStatus for String {
    fn deserialize_status(&self) -> Option<Status> {
        serde_json::from_slice(format!(r#""{self}""#).as_bytes()).ok()
    }
}

#[cfg(test)]
mod test {
    use super::{DeserializeStatus, Status};

    #[test]
    fn test_status_deserialize() {
        assert!(r#"-c"#.to_string().deserialize_status().is_none());
        assert!(r#""#.to_string().deserialize_status().is_none());
        // Status::On
        assert!(matches!("on".to_string().deserialize_status().unwrap(), Status::On));
        assert!(matches!("1".to_string().deserialize_status().unwrap(), Status::On));
        assert!(matches!("true".to_string().deserialize_status().unwrap(), Status::On));
        assert!(matches!("enable".to_string().deserialize_status().unwrap(), Status::On));
        // Status::Off
        assert!(matches!("off".to_string().deserialize_status().unwrap(), Status::Off));
        assert!(matches!("0".to_string().deserialize_status().unwrap(), Status::Off));
        assert!(matches!("false".to_string().deserialize_status().unwrap(), Status::Off));
        assert!(matches!("disable".to_string().deserialize_status().unwrap(), Status::Off));
    }

}