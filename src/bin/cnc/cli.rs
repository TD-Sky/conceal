use std::path::PathBuf;

use clap::Parser;

/// Put items into the recyle bin
#[derive(Parser)]
#[command(name = "cnc", version)]
pub struct Cli {
    /// The items to be discarded
    pub items: Vec<PathBuf>,
}
