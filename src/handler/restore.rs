use std::fmt::Write as _;
use std::io::{stdout, BufWriter, Write};
use std::process::{Command, Stdio};

use owo_colors::OwoColorize;
use trash::os_limited::restore_all;

use crate::{
    error::{Error, Result},
    handler::list::items,
    util::{self, time::local_datetime, tui::confirm_or_yes},
};

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
                src = src.display(),
            );
        });
    let options = options.trim_end();

    let mut finder = Command::new(finder)
        .args([
            "--multi",
            "--ansi",
            "--reverse",
            "--cycle",
            "--bind",
            "ctrl-a:select-all,ctrl-r:toggle-all",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| Error::FinderNotFound(finder))?;

    finder
        .stdin
        .as_mut()
        .unwrap()
        .write_all(options.as_bytes())?;

    let selected = finder.wait_with_output()?.stdout;
    let selected = String::from_utf8_lossy(&selected);

    // Select nothing,
    if selected.is_empty() {
        // Keep silent.
        return Ok(());
    }

    let selected: Vec<usize> = selected
        .lines()
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

    let mut stdout = BufWriter::new(stdout());
    for item in &items {
        let src = item.original_path();
        writeln!(&mut stdout, "{}", src.display())?;
    }
    stdout.flush()?;

    if confirm_or_yes("Restore above items?") {
        restore_all(items)?;
    }

    Ok(())
}
