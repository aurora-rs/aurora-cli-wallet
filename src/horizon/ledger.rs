use crate::config::AppConfig;
use crate::horizon::{
    execute_and_print_request, execute_and_print_stream_request, Paging, Streaming,
};
use anyhow::Result;
use convey::Output;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use stellar_horizon::resources::LedgerId;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Horizon ledger endpoints")]
pub enum LedgerCommand {
    All(AllLedgersCommand),
    Single(SingleLedgerCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a single ledger")]
pub struct SingleLedgerCommand {
    #[structopt(name = "LEDGER_ID", help = "The ledger id")]
    pub ledger_id: LedgerId,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of ledgers")]
pub struct AllLedgersCommand {
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: LedgerCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        LedgerCommand::All(cmd) => run_all(&mut out, &config, client, cmd).await,
        LedgerCommand::Single(cmd) => run_single(&mut out, &config, client, cmd).await,
    }
}

pub async fn run_all<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllLedgersCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::ledgers::all();
    execute_and_print_stream_request(
        &mut out,
        client,
        request,
        &command.paging,
        &command.streaming,
    )
    .await
}

pub async fn run_single<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: SingleLedgerCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::ledgers::single(command.ledger_id);
    execute_and_print_request(&mut out, client, request).await
}
