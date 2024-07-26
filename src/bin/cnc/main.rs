mod cli;

use std::io::stderr;
use std::process;

use clap::Parser;
use conceal::handler::put;

use self::cli::Cli;

fn main() {
    let cnc = Cli::parse();

    if let Err(e) = put(&cnc.entities) {
        e.handler("cnc", &mut stderr().lock());
        process::exit(1);
    };
}
