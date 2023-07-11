use std::env;

use trash::os_limited::list;
use trash::os_limited::purge_all;

use crate::error::Result;
use crate::utils::confirm;

pub fn clean(all: bool) -> Result<()> {
    let recycle_bin = dirs::data_local_dir()
        .map(|p| p.join("Trash"))
        .expect("recycle bin not found");
    let s = if all { "" } else { "for current directory" };
    let prompt = &format!("Clean {recycle_bin:?} {s}? (y/n) ");

    if !confirm(prompt) {
        return Ok(());
    }

    let mut items = list()?;
    if !all {
        let pwd = env::current_dir()?;
        items.retain(|it| it.original_parent == pwd);
    }
    purge_all(&items)?;

    Ok(())
}
