use std::io::{stdin, stdout, Read, Write};

pub fn confirm(prompt: String) -> bool {
    let mut stdout = stdout();
    write!(&mut stdout, "{prompt} ? (y/n) ").unwrap_or_default();
    stdout.flush().expect("Prompt failure");

    stdin()
        .bytes()
        .next()
        .and_then(|c| c.ok())
        .map(|c| c == b'y' || c == b'Y')
        .unwrap_or(false)
}
