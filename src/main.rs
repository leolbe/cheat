use std::path::{Path, PathBuf};

use crate::args::Args;
use clap::Parser;

mod args;
mod ops;
mod parse;

fn sheets_path(args: &Args) -> anyhow::Result<PathBuf> {
    args.sheets_path
        .as_ref()
        .map(|path| Ok(path.clone()))
        .unwrap_or_else(|| {
            let cheat_path = std::env::var("CHEAT_PATH").unwrap_or("~/.config/cheat".into());

            let cheat_path = Path::new(&cheat_path);
            Ok(cheat_path.join("sheets"))
        })
}

pub struct SectionPath {
    sheet: Box<str>,
    segments: Box<[Box<str>]>,
}

impl TryFrom<&str> for SectionPath {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        let mut segments = value.split('.');

        fn is_invalid_segment(segment: &&str) -> bool {
            segment.chars().any(char::is_control)
        }
        if let Some(invalid_segment) = segments.find(is_invalid_segment) {
            anyhow::bail!("Invalid section name '{}'", invalid_segment);
        }

        let sheet = segments
            .next()
            .ok_or(anyhow::anyhow!("Empty section path"))?;
        Ok(SectionPath {
            sheet: Box::from(sheet),
            segments: segments.map(Box::from).collect(),
        })
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let sheets_path = sheets_path(&args)?;

    if let Some(section_path) = args.section {
        let section_path = SectionPath::try_from(section_path.as_str())?;
        ops::print_sheet(&sheets_path, &section_path)?;
    } else {
        ops::list_sheets(&sheets_path)?;
    }

    Ok(())
}
