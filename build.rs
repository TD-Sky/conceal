#[path = "src/cli.rs"]
mod cli;

use self::cli::Cli;
use clap::CommandFactory;
use clap_complete::{
    generate_to,
    Shell::{Bash, Fish},
};
use clap_complete_nushell::Nushell;
use std::io::Result;

fn main() -> Result<()> {
    let cmd = &mut Cli::command();
    let name = "cnc";
    let dir = "complete";

    // Don't use clap's support for zsh.
    // TODO: zsh completions should be written manually in `complete/_cnc`
    generate_to(Bash, cmd, name, dir)?;
    generate_to(Fish, cmd, name, dir)?;
    generate_to(Nushell, cmd, name, dir)?;

    Ok(())
}
