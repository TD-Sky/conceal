use std::path::PathBuf;

use clap::Parser;

/// Put entities into the recyle bin
#[derive(Parser)]
#[command(name = "cnc", version)]
pub struct Cli {
    /// The entities to be discarded
    pub entities: Vec<PathBuf>,
}
