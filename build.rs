#[path = "src/bin/cnc/cli.rs"]
mod cnc;

#[allow(dead_code)]
#[path = "src/bin/conceal/cli.rs"]
mod conceal;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use cfg_aliases::cfg_aliases;
use clap::{Command, CommandFactory};
use clap_complete::Shell::{Bash, Fish, PowerShell, Zsh};
use clap_complete::generate_to;
use clap_complete_nushell::Nushell;

fn main() -> io::Result<()> {
    cfg_aliases! {
        freedesktop: {
            all(
                unix,
                not(target_os = "macos"),
                not(target_os = "ios"),
                not(target_os = "android")
        )}
    }

    if env::var_os("CONCEAL_GEN_COMPLETIONS").is_some_and(|s| s == "true") {
        generate_completions(&mut cnc::Cli::command(), "cnc")?;
        generate_completions(&mut conceal::Cli::command(), "conceal")?;
    }

    Ok(())
}

fn generate_completions(cmd: &mut Command, name: &str) -> io::Result<()> {
    let dir = &PathBuf::from("completions").join(name);
    fs::create_dir_all(dir)?;

    generate_to(Zsh, cmd, name, dir)?;
    generate_to(Bash, cmd, name, dir)?;
    generate_to(Fish, cmd, name, dir)?;
    generate_to(PowerShell, cmd, name, dir)?;
    generate_to(Nushell, cmd, name, dir)?;

    Ok(())
}
