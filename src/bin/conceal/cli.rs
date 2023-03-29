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
    /// List the discarded entities
    List {
        /// All discarded entities.
        /// If not, only list the entities discarded from current directory
        #[arg(long, short)]
        all: bool,
    },

    /// Restore entities discarded from the current directory
    Restore,

    /// Delete all the discarded entities permanently
    Clean,
}
