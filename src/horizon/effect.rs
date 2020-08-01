use crate::config::AppConfig;
use crate::horizon::{
    execute_and_print_page_request, execute_and_print_stream_request, Paging, Streaming,
};
use anyhow::Result;
use convey::Output;
use stellar_base::PublicKey;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use stellar_horizon::resources::LedgerId;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Horizon effect endpoints")]
pub enum EffectCommand {
    All(AllEffectsCommand),
    ForAccount(EffectsForAccountCommand),
    ForLedger(EffectsForLedgerCommand),
    ForOperation(EffectsForOperationCommand),
    ForTransaction(EffectsForTransactionCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions")]
pub struct AllEffectsCommand {
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions filtered by account")]
pub struct EffectsForAccountCommand {
    #[structopt(name = "ACCOUNT_ID", help = "The account id")]
    pub account_id: String,
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions filtered by ledger")]
pub struct EffectsForLedgerCommand {
    #[structopt(name = "LEDGER_ID", help = "The ledger id")]
    pub ledger_id: LedgerId,
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions filtered by operation")]
pub struct EffectsForOperationCommand {
    #[structopt(name = "OPERATION_ID", help = "The ledger id")]
    pub operation_id: String,
    #[structopt(flatten)]
    pub paging: Paging,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions filtered by transaction")]
pub struct EffectsForTransactionCommand {
    #[structopt(name = "TRANSACTION_ID", help = "The transaction id")]
    pub transaction_id: String,
    #[structopt(flatten)]
    pub paging: Paging,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: EffectCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        EffectCommand::All(cmd) => run_all(&mut out, &config, client, cmd).await,
        EffectCommand::ForAccount(cmd) => run_for_account(&mut out, &config, client, cmd).await,
        EffectCommand::ForLedger(cmd) => run_for_ledger(&mut out, &config, client, cmd).await,
        EffectCommand::ForOperation(cmd) => run_for_operation(&mut out, &config, client, cmd).await,
        EffectCommand::ForTransaction(cmd) => {
            run_for_transaction(&mut out, &config, client, cmd).await
        }
    }
}

pub async fn run_all<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllEffectsCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::effects::all();
    execute_and_print_stream_request(
        &mut out,
        client,
        request,
        &command.paging,
        &command.streaming,
    )
    .await
}

pub async fn run_for_account<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: EffectsForAccountCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let account = PublicKey::from_account_id(&command.account_id)?;
    let request = api::effects::for_account(&account);
    execute_and_print_stream_request(
        &mut out,
        client,
        request,
        &command.paging,
        &command.streaming,
    )
    .await
}

pub async fn run_for_ledger<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: EffectsForLedgerCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::effects::for_ledger(command.ledger_id);
    execute_and_print_stream_request(
        &mut out,
        client,
        request,
        &command.paging,
        &command.streaming,
    )
    .await
}

pub async fn run_for_operation<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: EffectsForOperationCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::effects::for_operation(command.operation_id);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}

pub async fn run_for_transaction<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: EffectsForTransactionCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::effects::for_transaction(command.transaction_id);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}
