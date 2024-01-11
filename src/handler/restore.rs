use std::fmt::Write as _;
use std::io::Write;
use std::process::{Command, Stdio};

use owo_colors::OwoColorize;
use trash::os_limited::restore_all;

use super::list::items;
use crate::error::{Error, Result};
use crate::util;
use crate::util::time::local_datetime;
use crate::util::tui::confirm;

pub fn restore(finder: &'static str) -> Result<()> {
    // Users only can restore files discarded under the current directory.
    let mut items = items(false)?;

    if items.is_empty() {
        return Ok(());
    }

    let iwidth = (items.len() as f64).log10().ceil() as usize;

    let mut options = String::new();
    items
        .iter()
        .enumerate()
        .rev() // For getting closest to the files discarded recently.
        .for_each(|(i, item)| {
            let src = item.original_path();
            let time = local_datetime(item.time_deleted).format(util::time::FORMAT);

            let _ = writeln!(
                options,
                "{i:0>iwidth$} {time} {src}",
                i = i.bright_purple().bold(),
                time = time.bright_yellow(),
                src = src.to_string_lossy(),
            );
        });
    let options = options.trim_end();

    // conceal list current directory trash
    // | <finder> --multi --ansi
    // | conceal restore selected trash
    let mut finder = Command::new(finder)
        .args(["-m", "--ansi", "--cycle"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| Error::FinderNotFound(finder))?;

    finder
        .stdin
        .as_mut()
        .unwrap()
        .write_all(options.as_bytes())?;

    // Linux shouldn't have UTF-8 problems
    let selected = String::from_utf8(finder.wait_with_output()?.stdout).unwrap();

    // Select nothing,
    if selected.is_empty() {
        // Keep silent.
        return Ok(());
    }

    let selected: Vec<usize> = selected
        .trim_end() // Tail '\n' is forbidden for `split` here.
        .split('\n')
        .map(|item| {
            // There're white spaces at the left side of index
            // because of right aligned.
            item.trim_start()
                .split_once(' ') // white space between index and date time
                .and_then(|(index, _)| index.parse().ok())
                .unwrap() // will definitely succeed
        })
        .rev() // The selected items is inverse by the index, rearrange them.
        .collect();

    // Reserve the selected items in `items`.
    let len = selected.len();
    for (low, high) in selected.into_iter().enumerate() {
        items.swap(low, high);
    }
    items.truncate(len);

    // Ask the users if they want to restore.
    let mut prompt = String::new();
    for item in &items {
        let src = item.original_path();
        let _ = writeln!(prompt, "{src}", src = src.to_string_lossy());
    }
    prompt += "\nRestore above items? (y/n) ";

    if confirm(&prompt) {
        restore_all(items)?;
    }

    Ok(())
}
