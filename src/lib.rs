pub mod commands;
pub mod error;
mod utils {
    mod alloc;
    pub mod time;
    #[cfg(any(freedesktop, target_os = "windows"))]
    pub mod trash;
    pub mod tui;
}
pub mod cli {
    pub mod cnc;
    pub mod completion;
    pub mod conceal;

    pub use completion::*;
}

pub use self::error::{Error, Result};
