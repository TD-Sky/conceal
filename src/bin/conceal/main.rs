mod cli;

use clap::Parser;

use cli::Cli;

#[cfg(any(freedesktop, target_os = "windows"))]
fn main() {
    use std::process;

    use conceal::handler;

    use cli::SubCommand;

    let cli = Cli::parse();

    use SubCommand::*;
    let result = match cli.command {
        List { all } => handler::list(all),
        Restore { finder } => handler::restore(finder.cmd()),
        Delete { finder, all } => handler::delete(finder.cmd(), all),
        Clean { all } => handler::clean(all),
    };

    if let Err(e) = result {
        e.print("conceal");
        process::exit(1);
    }
}

#[cfg(target_os = "macos")]
fn main() {
    Cli::parse();
}
