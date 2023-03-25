mod cli;
use self::cli::Cli;

use anyhow::Result;
use clap::Parser;
use conceal::handlers::put;

fn main() -> Result<()> {
    let cnc = Cli::parse();
    put(&cnc.entities)
}
