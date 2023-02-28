mod cli;
mod handlers;
mod utils;

use self::cli::*;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(Put(cli.put_args));

    use SubCommand::*;
    match command {
        Put(PutArgs { items }) => handlers::put(&items),

        List => handlers::list(),

        Restore => handlers::restore(),

        Empty => handlers::empty(),
    }
}
