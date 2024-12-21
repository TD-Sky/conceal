mod cli;

use clap::Parser;

use cli::Cli;

#[cfg(freedesktop)]
fn main() {
    use std::process;

    use conceal::handler;

    use cli::SubCommand;

    let cli = Cli::parse();

    use SubCommand::*;
    let result = match cli.command {
        List { all } => handler::list(all),
        Restore { finder } => handler::restore(finder.cmd()),
        Clean { all } => handler::clean(all),
    };

    if let Err(e) = result {
        e.print("conceal");
        process::exit(1);
    }
}

#[cfg(not(freedesktop))]
fn main() {
    Cli::parse();
}
