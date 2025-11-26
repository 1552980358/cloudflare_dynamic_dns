use std::process::exit;

pub(super) mod args {

    pub const LONG: &'static str = "--help";
    pub const SHORT: &'static str = "-h";
    pub const SYMBOL: &'static str = "?";

}

fn message_str() -> String {
    String::from(
        concat!(
            "Usage: cloudflare_dynamic_dns [options...] \n",
            "Options: \n",
            "  --help, -h, ? \n",
            "      Display all available command line options \n",
            "  --cloudflare, -cf <path> \n", 
            "      Read cloudflare json configurations from <path> \n",
            "  --config, -c <path> \n",
            "      Read configuration from <path> \n",
            "\n",
        )
    )
}

pub(super) fn print_message() {
    println!("{}", message_str());
    // Exit normally
    exit(0);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_message_str() {
        println!("{}", super::message_str());
    }

}