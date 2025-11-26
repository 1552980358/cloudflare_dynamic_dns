use std::process::exit;

pub(super) mod args {

    pub const LONG: &'static str = "--help";
    pub const SHORT: &'static str = "-h";
    pub const SYMBOL: &'static str = "?";

}

fn message_str() -> String {
    String::from(
        concat!(
            "Usage: cloudflare_dynamic_dns [options...]\n",
            "Options: \n",
            "  --help, -h, ? \n",
            "      Display all available command line options.\n",
            "\n",
        )
    )
}

pub(super) fn print_message() {
    println!("{}", message_str());
    // Exit normally
    exit(0);
}