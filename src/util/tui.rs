use std::io::{self, Read, Write, stdin, stdout};

trait ReadExt {
    fn read_u8(&mut self) -> Option<u8>;
}

impl<T> ReadExt for T
where
    T: Read,
{
    fn read_u8(&mut self) -> Option<u8> {
        let mut byte = 0;
        self.read_exact(std::slice::from_mut(&mut byte)).ok()?;
        Some(byte)
    }
}

fn confirm(prompt: &str, default: bool) -> bool {
    || -> io::Result<()> {
        let mut stdout = stdout();
        stdout.write_all(prompt.as_bytes())?;
        stdout.flush()
    }()
    .expect("prompt failure");

    match stdin().read_u8() {
        Some(b'\n') => default,
        Some(c) => c == b'y' || c == b'Y',
        None => false,
    }
}

pub fn confirm_or_yes(prompt: &str) -> bool {
    confirm(&format!("\n{prompt} [Y/n] "), true)
}

pub fn confirm_or_no(prompt: &str) -> bool {
    confirm(&format!("\n{prompt} [y/N] "), false)
}
