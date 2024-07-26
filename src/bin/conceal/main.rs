mod cli;

use std::io::stderr;
use std::process;

use clap::Parser;
use conceal::handler;

use self::cli::{Cli, SubCommand};

fn main() {
    let cli = Cli::parse();

    use SubCommand::*;
    let result = match cli.command {
        List { all } => handler::list(all),
        Restore { finder } => handler::restore(finder.cmd()),
        Clean { all } => handler::clean(all),
    };

    if let Err(e) = result {
        e.handler("conceal", &mut stderr().lock());
        process::exit(1);
    }
}
