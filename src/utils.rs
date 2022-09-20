use std::io::{stdin, stdout, Read, Write};

pub fn confirm(prompt: String) -> bool {
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
