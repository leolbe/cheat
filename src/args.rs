use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Cheatsheet name followed by zero or more dot-separated section specifiers, e.g.
    /// 'vim.plugins.telescope'
    #[arg()]
    pub section: Option<String>,
    /// Look for cheatsheets here instead of the default path.
    #[arg(short = 'p')]
    pub sheets_path: Option<PathBuf>,
}
