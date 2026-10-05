use clap::Parser;
use crate::args::Args;

mod args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let user = std::env::var("USER")?;

    println!("{}, {}!", args.greeting, user);

    anyhow::Ok(())
}
