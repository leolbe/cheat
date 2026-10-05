use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(default_value = "Hello")]
    pub greeting: String,
}
