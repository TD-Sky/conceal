mod cli;
use self::cli::Cli;
use self::cli::SubCommand;

use clap::Parser;
use conceal::handlers;
use std::io::stderr;
use std::process;

fn main() {
    let cli = Cli::parse();

    use SubCommand::*;
    let result = match cli.command {
        List { all } => handlers::list(all),
        Restore { finder } => handlers::restore(finder.cmd()),
        Clean { all } => handlers::clean(all),
    };

    if let Err(e) = result {
        e.handler("conceal", &mut stderr().lock());
        process::exit(1);
    }
}
