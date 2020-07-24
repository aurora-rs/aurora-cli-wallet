use crate::config::AppConfig;
use anyhow::Result;
use convey::Output;
use structopt::StructOpt;

mod account;
mod ledger;
mod server;

#[derive(Debug, StructOpt)]
#[structopt(about = "Call Horizon endpoints")]
pub struct HorizonCommand {
    #[structopt(short, long, global = true, help = "Which Horizon server to use")]
    pub server: Option<String>,
    #[structopt(subcommand)]
    pub command: HorizonInnerCommand,
}

#[derive(Debug, StructOpt)]
pub enum HorizonInnerCommand {
    Server(server::ServerCommand),
    #[structopt(flatten)]
    NonServer(HorizonNonServerCommand),
}

#[derive(Debug, StructOpt)]
pub enum HorizonNonServerCommand {
    Account(account::AccountCommand),
    Ledger(ledger::LedgerCommand),
}

pub async fn run_command(
    mut out: &mut Output,
    mut config: &mut AppConfig,
    command: HorizonCommand,
) -> Result<()> {
    match command.command {
        HorizonInnerCommand::Server(cmd) => server::run_command(&mut out, &mut config, cmd),
        HorizonInnerCommand::NonServer(cmd) => {
            let server_name = command.server.unwrap_or(config.default_server.clone());
            let client = config.horizon_client_for_server(&server_name)?;
            match cmd {
                HorizonNonServerCommand::Account(cmd) => {
                    account::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Ledger(cmd) => {
                    ledger::run_command(&mut out, &config, &client, cmd).await
                }
            }
        }
    }
}
