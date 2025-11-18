use std::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use pulldown_cmark::{Event, Tag};

/// Check local file links in Markdown files
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Files to check
    #[arg()]
    pub file: Vec<String>,

    #[arg(short, long)]
    pub verbose: bool,
}

fn check_link(link: &str, path: &str, range: Range<usize>, verbose: bool) -> Result<bool> {
    if link.starts_with("http://") || link.starts_with("https://") || link.starts_with("mailto:") {
        return Ok(true);
    }
    let link_str = &link[0..link.rfind('#').unwrap_or(link.len())];
    let link = if let Some(parent) = PathBuf::from(path).parent() {
        parent.join(link_str)
    } else {
        PathBuf::from(link_str)
    };
    if !Path::new(&link)
        .try_exists()
        .with_context(|| format!("Couldn't check if path exists: {link_str}"))?
    {
        println!("✗ {}:{}:{} {}", path, range.start, range.end, link_str);
        return Ok(false);
    } else if verbose {
        println!("✓ {}:{}:{} {}", path, range.start, range.end, link_str);
    }
    Ok(true)
}

fn check_file(path: &str, verbose: bool) -> Result<bool> {
    let mut ok = true;
    let md = std::fs::read_to_string(path).with_context(|| format!("Couldn't read {path}"))?;
    for (ev, range) in pulldown_cmark::Parser::new(&md).into_offset_iter() {
        if let Event::Start(Tag::Link { dest_url, .. } | Tag::Image { dest_url, .. }) = ev {
            ok &= check_link(dest_url.as_ref(), path, range, verbose)?;
        };
    }
    Ok(ok)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let files = args.file.into_iter();
    let sum: i32 = files
        .map(|f| Ok(i32::from(!(check_file(&f, args.verbose)?))))
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .sum();
    std::process::exit(std::cmp::min(sum, 125));
}

#[cfg(test)]
mod tests {
    use super::check_link;
    #[test]
    fn test_anchor() {
        assert!(check_link("README.md#foo", ".", 0..0, false).unwrap());
    }
}
