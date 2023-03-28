pub mod time;

use std::io::Read;
use std::io::Write;
use std::io::{stdin, stdout};

pub fn confirm(prompt: &str) -> bool {
    let mut stdout = stdout();
    stdout.write_all(prompt.as_bytes()).expect("Prompt failure");
    stdout.flush().expect("Prompt failure");

    stdin()
        .bytes()
        .next()
        .and_then(|c| c.ok())
        .map(|c| c == b'y' || c == b'Y')
        .unwrap_or(false)
}
