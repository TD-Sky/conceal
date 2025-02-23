use std::env;
use std::fmt::Write as _;
use std::io::{self, BufWriter, Write, stdout};
use std::process::{Command, Stdio};

use owo_colors::OwoColorize;
use trash::TrashItem;

use crate::{
    error::{Error, Result},
    util::{self, time::local_datetime},
};

pub fn items(all: bool) -> Result<Vec<TrashItem>> {
    let pwd = (!all).then(env::current_dir).transpose()?;

    let mut items = trash::os_limited::list()?;
    if let Some(prefix) = pwd.as_deref() {
        items.retain_mut(|item| {
            if let Ok(p) = item.original_parent.strip_prefix(prefix) {
                item.original_parent = p.into();
                true
            } else {
                false
            }
        })
    }
    items.sort_by_key(|item| item.time_deleted);

    Ok(items)
}

pub fn render<'a>(
    w: &mut dyn Write,
    items: impl IntoIterator<Item = &'a TrashItem>,
) -> io::Result<()> {
    for item in items {
        let time = local_datetime(item.time_deleted).format(util::time::FORMAT);
        let src = item.original_path();
        writeln!(
            w,
            "{time} {src}",
            time = time.bright_yellow(),
            src = src.display()
        )?;
    }

    w.flush()
}

pub fn select_items(finder: &'static str, all: bool) -> Result<Vec<TrashItem>> {
    let mut items = items(all)?;

    if items.is_empty() {
        return Ok(vec![]);
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
        return Ok(vec![]);
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

    Ok(items)
}
