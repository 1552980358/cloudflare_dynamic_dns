use super::argument::Argument;

pub(super) trait GetProxied {
    fn get_proxied(&self) -> Option<bool>;
}

impl GetProxied for Vec<Argument> {
    fn get_proxied(&self) -> Option<bool> {
        self.iter()
            .find_map(|argument|
                if let Argument::Proxied(proxied) = argument { Some(*proxied) }
                else { None }
            )
    }
}