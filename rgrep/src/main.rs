use anyhow::Result;
use clap::Parser;
use rgrep::*;

fn main() -> Result<()> {
    let config: Cli = Cli::parse();
    config.match_with_default_strategy()?;

    Ok(())
}
