use conceal::cli::conceal::Cli;

#[cfg(any(freedesktop, target_os = "windows"))]
fn main() {
    use std::process;

    use conceal::{cli::conceal::SubCommand, commands};

    let cli = Cli::parse();

    use SubCommand::*;
    let result = match cli.command {
        List(args) => commands::list(&args),
        Restore(args) => commands::restore(&args),
        Delete(args) => commands::delete(&args),
        Clean(args) => commands::clean(&args),
        Completion(args) => {
            print!("{}", Cli::completion_script(args.shell()));
            Ok(())
        }
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
