use usage_rs::{Subcommands, ValueEnum};

use crate::cli::Completion;

/// Operate the trash bin
#[derive(usage_rs::Cli)]
#[usage(bin = "conceal", version, completion)]
pub struct Cli {
    #[usage(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommands)]
pub enum SubCommand {
    /// List the discarded items
    #[usage(visible_alias = "ls")]
    List(ListArgs),

    /// Restore items discarded from the current directory
    #[usage(visible_alias = "rs")]
    Restore(RestoreArgs),

    /// Delete the discarded items permanently
    #[usage(visible_alias = "del")]
    Delete(DeleteArgs),

    /// Clean the discarded items permanently
    Clean(CleanArgs),

    /// Print a completion script
    Completion(Completion),
}

#[derive(usage_rs::Args)]
pub struct ListArgs {
    /// All discarded items.
    /// If not, only list the items discarded under current directory
    #[usage(long, short)]
    pub all: bool,

    /// Clean items discarded since the point (including this point)
    /// (<TIME> follows systemd.time specifications)
    #[usage(long, short, value_name = "TIME")]
    pub since: Option<String>,

    /// Clean items discarded until the point (excluding this point)
    /// (<TIME> follows systemd.time specifications)
    #[usage(long, short, value_name = "TIME")]
    pub until: Option<String>,
}

#[derive(usage_rs::Args)]
pub struct RestoreArgs {
    #[usage(
        long,
        value_enum,
        default_value_t,
        default = "fzf",
        env = "CONCEAL_FINDER"
    )]
    pub finder: Finder,
}

#[derive(usage_rs::Args)]
pub struct DeleteArgs {
    #[usage(
        long,
        value_enum,
        default_value_t,
        default = "fzf",
        env = "CONCEAL_FINDER"
    )]
    pub finder: Finder,

    /// All discarded items.
    /// If not, only emerge the items discarded under current directory
    #[usage(long, short)]
    pub all: bool,
}

#[derive(usage_rs::Args)]
pub struct CleanArgs {
    /// All discarded items.
    /// If not, only clean the items discarded under current directory
    #[usage(long, short)]
    pub all: bool,

    /// Clean items discarded since the point (including this point)
    /// (<TIME> follows systemd.time specifications)
    #[usage(long, short, value_name = "TIME")]
    pub since: Option<String>,

    /// Clean items discarded until the point (excluding this point)
    /// (<TIME> follows systemd.time specifications)
    #[usage(long, short, value_name = "TIME")]
    pub until: Option<String>,
}

#[derive(Default, ValueEnum, Clone, Copy)]
pub enum Finder {
    Skim,
    #[default]
    Fzf,
}

impl std::fmt::Display for Finder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Skim => "skim",
            Self::Fzf => "fzf",
        })
    }
}

impl Finder {
    pub fn cmd(self) -> &'static str {
        match self {
            Self::Skim => "sk",
            Self::Fzf => "fzf",
        }
    }
}
