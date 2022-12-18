use std::ops::Range;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use pulldown_cmark::{Event, Tag};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

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
    if link.starts_with("http://") || link.starts_with("https://") {
        return Ok(true);
    }
    if !Path::new(link)
        .try_exists()
        .with_context(|| format!("Couldn't check if path exists: {}", link))?
    {
        println!("✗ {}:{}:{} {}", path, range.start, range.end, link);
        return Ok(false);
    } else if verbose {
        println!("✓ {}:{}:{} {}", path, range.start, range.end, link);
    }
    Ok(true)
}

fn check_file(path: &str, verbose: bool) -> Result<bool> {
    let mut ok = true;
    let md = std::fs::read_to_string(path).with_context(|| format!("Couldn't read {}", path))?;
    for (ev, range) in pulldown_cmark::Parser::new(&md).into_offset_iter() {
        if let Event::Start(Tag::Link(_, link, _) | Tag::Image(_, link, _)) = ev {
            ok &= check_link(&link, path, range, verbose)?;
        };
    }
    Ok(ok)
}

fn main() -> Result<()> {
    let args = Args::parse();

    #[cfg(not(feature = "parallel"))]
    let files = args.file.into_iter();
    #[cfg(feature = "parallel")]
    let files = args.file.into_par_iter();

    let sum: i32 = files
        .map(|f| Ok(i32::from(!(check_file(&f, args.verbose)?))))
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .sum();

    std::process::exit(std::cmp::min(sum, 125));
}
