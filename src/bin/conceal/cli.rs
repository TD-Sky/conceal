use clap::Parser;
use clap::Subcommand;

/// Operate the recycle bin
#[derive(Parser)]
#[command(name = "conceal", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// List all the discarded entities
    List,

    /// Restore entities discarded under the current directory
    Restore,

    /// Delete all the discarded entities permanently
    Clean,
}
