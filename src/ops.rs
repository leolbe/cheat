use crate::SectionPath;
use crate::args::Args;
use crate::parse::{Entry, Section};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::path::Path;

fn strip_toml_extension(mut file_name: String) -> Option<String> {
    const FILE_EXTENSION: &str = ".toml";

    if file_name.ends_with(FILE_EXTENSION) {
        file_name.truncate(file_name.len() - FILE_EXTENSION.len());
        Some(file_name)
    } else {
        None
    }
}

pub fn list_sheets(sheets_dir: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(sheets_dir)?;
    let entries: Result<Vec<_>, _> = sheets_dir.read_dir()?.collect();
    let entries = entries?;

    let sheets: anyhow::Result<Vec<_>> = entries
        .into_par_iter()
        .filter_map(|entry| {
            let file_type = match entry.file_type() {
                Ok(t) => t,
                Err(err) => return Some(Err(anyhow::anyhow!(err))),
            };
            if file_type.is_file() {
                let file_name = match entry.file_name().to_str() {
                    Some(name) => name.to_string(),
                    None => {
                        return Some(Err(anyhow::anyhow!("File name contains invalid unicode")));
                    }
                };
                strip_toml_extension(file_name).map(Ok)
            } else {
                None
            }
        })
        .collect();
    let mut sheets = sheets?;
    sheets.sort();

    if let Some(sheets_dir) = sheets_dir.to_str() {
        println!("Available sheets in '{}':", sheets_dir);
    } else {
        println!("Available sheets");
    }
    sheets.iter().for_each(|sheet| println!("{}", sheet));
    Ok(())
}

pub fn print_text_entries(section: &Section) {
    let text_entries = section
        .iter()
        .filter_map(|(key, entry)| entry.as_text().map(|text| (key, text)));

    let column_width = text_entries
        .clone()
        .map(|(key, _)| key.len())
        .max()
        .unwrap_or(0);

    for (key, entry) in text_entries {
        println!("{:column_width$} {}", key, entry);
    }
}

pub fn print_section(section: &Section, section_name: &str, depth: usize, recursive: bool) {
    if depth > 0 {
        println!();
    }
    // format trick to print `depth` number of '='
    println!("{:=<depth$} {section_name}", "", depth = depth + 1);
    print_text_entries(section);

    if recursive {
        let subsection_entries = section
            .iter()
            .filter_map(|(key, entry)| entry.as_subsection().map(|subsection| (key, subsection)));

        for (key, subsection) in subsection_entries {
            print_section(subsection, &key, depth + 1, recursive);
        }
    }
}

pub fn print_sheet(
    sheets_dir: &Path,
    section_path: &SectionPath,
    args: &Args,
) -> anyhow::Result<()> {
    let sheet_path = sheets_dir.join(format!("{}.toml", section_path.sheet));
    let sheet_content = std::fs::read_to_string(sheet_path)?;
    let sheet: Section = toml::from_str(&sheet_content)?;

    let mut section = &sheet;
    let mut section_name = &section_path.sheet;
    for path_segment in &section_path.segments {
        let Some(Entry::Subsection(subsection)) = section.get(path_segment) else {
            anyhow::bail!("No subsection with name '{}'", path_segment);
        };
        section = subsection;
        section_name = path_segment;
    }

    print_section(section, &section_name, 0, !args.section_only);
    Ok(())
}
