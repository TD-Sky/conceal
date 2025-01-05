mod cli;

use std::process;

use clap::Parser;
use conceal::handler::put;

use self::cli::Cli;

fn main() {
    let cnc = Cli::parse();

    if let Err(e) = put(&cnc.items) {
        e.print("cnc");
        process::exit(1);
    };
}
