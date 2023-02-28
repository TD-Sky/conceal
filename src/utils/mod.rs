pub mod time;

use std::io::{stdin, stdout, Read, Write};
use trash::TrashItem;

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

pub fn valid_part(items: Vec<TrashItem>) -> impl Iterator<Item = TrashItem> {
    items.into_iter().filter(|item| {
        let src = item.original_path();

        if src.is_symlink() {
            src.exists()
        } else {
            true
        }
    })
}
