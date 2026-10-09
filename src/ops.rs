use crate::SectionPath;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::path::Path;

pub fn is_toml_file(file_name: &str) -> bool {
    file_name.ends_with(".toml")
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
                    None => return Some(Err(anyhow::anyhow!("File name contains invalid unicode"))),
                };
                is_toml_file(&file_name).then_some(Ok(file_name))
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

pub fn print_sheet(sheets_dir: &Path, section_path: &SectionPath) -> anyhow::Result<()> {
    todo!()
}
