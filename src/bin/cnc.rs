use std::process;

use conceal::{cli::cnc::Cli, commands::put};

fn main() {
    let cnc = Cli::parse();

    if let Err(e) = put(&cnc.items) {
        e.print("cnc");
        process::exit(1);
    };
}
