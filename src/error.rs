#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
use std::env;
use std::io::{self, stderr, Write};
#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
use std::path::Path;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("`trash` error: {0:#?}")]
    Trash(#[from] trash::Error),

    #[error("`{0}` not found")]
    FinderNotFound(&'static str),

    #[error("{0}")]
    Msg(&'static str),
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Self::Msg(msg)
    }
}

impl Error {
    pub fn print(&self, binary: &'static str) {
        let mut stderr = stderr();

        match self {
            #[cfg(all(
                unix,
                not(target_os = "macos"),
                not(target_os = "ios"),
                not(target_os = "android")
            ))]
            Self::Trash(trash::Error::FileSystem { path, source: e }) => {
                let pwd = env::current_dir().unwrap();
                if let Ok(relative_path) = path.strip_prefix(pwd).map(Path::display) {
                    writeln!(stderr, "{binary}: {relative_path}: {e}").unwrap();
                }
            }
            _ => {
                writeln!(stderr, "{binary}: {self}").unwrap();
            }
        }
    }
}
