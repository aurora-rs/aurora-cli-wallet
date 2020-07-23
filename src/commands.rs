use crate::account;
use anyhow::Result;
use convey::Output;
use structopt::StructOpt;

use crate::config::AppConfig;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum OutputFormat {
        Text,
        Json,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Stellar cli wallet for hackers.",
    name = "aurora",
    author = "Francesco Ceccon <francesco@ceccon.me>",
    version = crate::VERSION
)]
pub struct Aurora {
    #[structopt(short, long, possible_values = &OutputFormat::variants(), case_insensitive=true, help = "Output format")]
    pub output: Option<OutputFormat>,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Account(account::AccountCommand),
}

pub fn run_command(
    mut out: &mut Output,
    mut config: &mut AppConfig,
    command: Command,
) -> Result<()> {
    match command {
        Command::Account(cmd) => account::run_command(&mut out, &mut config, cmd),
    }
}
