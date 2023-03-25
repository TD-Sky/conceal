mod cli;
use self::cli::Cli;
use self::cli::SubCommand;

use clap::Parser;
use conceal::handlers;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    use SubCommand::*;
    match cli.command {
        List => handlers::list(),
        Restore => handlers::restore(),
        Clean => handlers::clean(),
    }
}
