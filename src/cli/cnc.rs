use std::path::PathBuf;

/// Put items into the trash bin
#[derive(usage_rs::Cli)]
#[usage(bin = "cnc", version)]
pub struct Cli {
    /// The items to be discarded
    pub items: Vec<PathBuf>,
}
