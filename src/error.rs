use std::io;

pub type Result<T, E = Error> = std::result::Result<T, E>;

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
        match self {
            #[cfg(freedesktop)]
            Self::Trash(trash::Error::FileSystem { path, source: e }) => {
                let pwd = std::env::current_dir().unwrap();
                let path = path.strip_prefix(pwd).unwrap_or(path);
                eprintln!("{binary}: {}: {e}", path.display());
            }
            _ => eprintln!("{binary}: {self}"),
        }
    }
}
