pub mod time;

use std::io;
use std::io::{stdin, stdout};
use std::io::{Read, Write};

pub fn confirm(prompt: &str) -> bool {
    let mut stdout = stdout();
    || -> io::Result<()> {
        stdout.write_all(prompt.as_bytes())?;
        stdout.flush()
    }()
    .expect("Prompt failure");

    stdin()
        .bytes()
        .next()
        .and_then(|c| c.ok())
        .map(|c| c == b'y' || c == b'Y')
        .unwrap_or(false)
}
