#[path = "src/bin/cnc/cli.rs"]
mod cnc;

#[path = "src/bin/conceal/cli.rs"]
mod conceal;

use clap::{Command, CommandFactory};
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};
use clap_complete_nushell::Nushell;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    generate_completions(&mut cnc::Cli::command(), "cnc")?;
    generate_completions(&mut conceal::Cli::command(), "conceal")?;

    Ok(())
}

fn generate_completions(cmd: &mut Command, name: &'static str) -> io::Result<()> {
    let dir = &PathBuf::from("completions").join(name);

    generate_to(Zsh, cmd, name, dir)?;
    generate_to(Bash, cmd, name, dir)?;
    generate_to(Fish, cmd, name, dir)?;
    generate_to(Nushell, cmd, name, dir)?;

    Ok(())
}
