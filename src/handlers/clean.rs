use crate::error::Result;
use crate::utils::confirm;
use std::env;
use trash::os_limited::list;
use trash::os_limited::purge_all;

pub fn clean() -> Result<()> {
    let home = env::var("HOME").expect("HOME not found");
    let prompt = &format!("Clean {home}/.local/share/Trash ? (y/n) ");

    if confirm(prompt) {
        let items = list()?;
        purge_all(&items)?;
    }

    Ok(())
}
