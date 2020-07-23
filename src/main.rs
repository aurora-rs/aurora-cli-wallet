#[macro_use]
extern crate anyhow;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate convey;
#[macro_use]
extern crate clap;

use convey::Output;
use structopt::StructOpt;

use anyhow::Result;

mod account;
mod commands;
mod config;
mod error;
mod horizon;
mod render;

use crate::commands::OutputFormat;
use crate::error::Error;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = config::load()?;
    let command = commands::Aurora::from_args();
    let mut out = new_output(&command.output)?;
    commands::run_command(&mut out, &mut app, command.command).await
}

fn new_output(format: &Option<OutputFormat>) -> Result<Output> {
    let target = match format {
        Some(OutputFormat::Text) => convey::human::stdout().map_err(Error::Convey)?,
        Some(OutputFormat::Json) => convey::json::stdout().map_err(Error::Convey)?,
        None => {
            if atty::is(atty::Stream::Stdout) {
                convey::human::stdout().map_err(Error::Convey)?
            } else {
                convey::json::stdout().map_err(Error::Convey)?
            }
        }
    };
    Ok(convey::new().add_target(target).map_err(Error::Convey)?)
}
