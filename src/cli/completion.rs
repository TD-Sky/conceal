use usage_rs::{Args, complete::Shell};

#[derive(Args)]
pub struct Completion {
    /// Which shell to generate for
    #[usage(long, choices("bash", "zsh", "fish", "nu", "powershell"))]
    shell: String,
}

impl Completion {
    pub fn shell(&self) -> Shell {
        match self.shell.as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "nu" => Shell::Nu,
            "powershell" => Shell::PowerShell,
            _ => unreachable!(),
        }
    }
}
