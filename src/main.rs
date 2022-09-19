mod cli;
mod empty;
mod list;
mod put;
mod restore;
mod utils;

use self::{cli::*, empty::empty, list::list, put::put, restore::restore};
use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    use SubCommand::*;

    let cli = Cli::parse();

    let command = cli.command.unwrap_or_else(|| Put(cli.put_args));

    match command {
        Put(PutArgs { items }) => put(items),

        List => list(),

        Restore => restore(),

        Empty => empty(),
    }
}
