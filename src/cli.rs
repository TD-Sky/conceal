use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cnc", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommand>,

    #[command(flatten)]
    pub put_args: PutArgs,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// [default] Throw the files in the trash bin
    Put(PutArgs),

    /// List all the discarded files
    List,

    /// Restore files discarded under the current directory
    Restore,

    /// Delete all the discarded files permanently
    Empty,
}

#[derive(Args)]
pub struct PutArgs {
    /// The files to be discarded
    pub items: Vec<PathBuf>,
}
