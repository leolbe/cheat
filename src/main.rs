use std::env::VarError;
use std::path::{Path, PathBuf};

use crate::args::Args;
use clap::Parser;

mod args;
mod ops;

fn sheets_path(args: &Args) -> anyhow::Result<PathBuf> {
    if let Some(sheets_path) = args.sheets_path.as_ref() {
        return Ok(sheets_path.clone());
    }

    match std::env::var("CHEAT_PATH") {
        Ok(cheat_path) => {
            if !cheat_path.is_empty() {
                let cheat_path = Path::new(&cheat_path);
                return Ok(cheat_path.join("sheets/"));
            }
        }
        Err(VarError::NotPresent) => {}
        Err(err) => {
            anyhow::bail!(err);
        }
    }

    let config_dir = dirs::config_dir().ok_or(anyhow::anyhow!("Failed to retrieve config directory"))?;
    Ok(config_dir.join("cheat/sheets/"))
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
        if let Some(invalid_segment) = segments.clone().find(is_invalid_segment) {
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
