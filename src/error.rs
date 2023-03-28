use std::env;
use std::io;
use std::io::Write;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Trash(#[from] trash::Error),

    #[error(transparent)]
    Time(#[from] time::error::IndeterminateOffset),

    #[error("`skim` isn't found")]
    SkimNotFound,

    #[error("unable to acquire stdin of `skim`")]
    SkimStdin,

    #[error("{0}")]
    Msg(&'static str),
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Self::Msg(msg)
    }
}

impl Error {
    #[inline]
    pub fn handler(&self, binary: &'static str, writer: &mut dyn Write) {
        if let Self::Trash(trash::Error::Io { path, base: e }) = self {
            let pwd = env::current_dir().unwrap();
            if let Ok(relative_path) = path.strip_prefix(pwd) {
                writeln!(writer, "{binary}: {relative_path:?}: {e}").unwrap();
                return;
            }
        }

        writeln!(writer, "{binary}: {self}").unwrap();
    }
}
