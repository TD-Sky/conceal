use std::io;

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
        match self {
            #[cfg(free_unix)]
            Self::Trash(trash::Error::FileSystem { path, source: e }) => {
                use std::path::Path;

                let pwd = std::env::current_dir().unwrap();
                if let Ok(relative_path) = path.strip_prefix(pwd).map(Path::display) {
                    eprintln!("{binary}: {relative_path}: {e}");
                }
            }
            _ => eprintln!("{binary}: {self}"),
        }
    }
}
