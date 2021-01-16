mod cli;
mod config;
mod run;
mod runner;

use cli::Root;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    Root::new().execute()
}