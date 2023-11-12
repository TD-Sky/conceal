mod cli;
use self::cli::Cli;

use clap::Parser;
use conceal::handler::put;
use std::io::stderr;
use std::process;

fn main() {
    let cnc = Cli::parse();

    if let Err(e) = put(&cnc.entities) {
        e.handler("cnc", &mut stderr().lock());
        process::exit(1);
    };
}
