use std::fmt;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

/// Operate the recycle bin
#[derive(Parser)]
#[command(name = "conceal", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// List the discarded items
    #[command(visible_alias = "ls")]
    List {
        /// All discarded items.
        /// If not, only list the items discarded under current directory
        #[arg(long, short)]
        all: bool,
    },

    /// Restore items discarded from the current directory
    #[command(visible_alias = "rs")]
    Restore {
        #[arg(long, default_value_t, env = "CONCEAL_FINDER")]
        finder: Finder,
    },

    /// Delete the discarded items permanently
    #[command(visible_alias = "del")]
    Delete {
        #[arg(long, default_value_t, env = "CONCEAL_FINDER")]
        finder: Finder,

        /// All discarded items.
        /// If not, only emerge the items discarded under current directory
        #[arg(long, short)]
        all: bool,
    },

    /// Clean the discarded items permanently
    Clean {
        /// All discarded items.
        /// If not, only clean the items discarded under current directory
        #[arg(long, short)]
        all: bool,
    },
}

#[derive(Default, ValueEnum, Clone, Copy)]
pub enum Finder {
    Skim,
    #[default]
    Fzf,
}

impl fmt::Display for Finder {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Skim => "skim",
            Self::Fzf => "fzf",
        })
    }
}

impl Finder {
    #[inline]
    pub fn cmd(self) -> &'static str {
        match self {
            Self::Skim => "sk",
            Self::Fzf => "fzf",
        }
    }
}
