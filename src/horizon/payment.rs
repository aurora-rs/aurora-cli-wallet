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
#[structopt(about = "Horizon payment endpoints")]
pub enum PaymentCommand {
    All(AllPaymentsCommand),
    ForAccount(PaymentsForAccountCommand),
    ForLedger(PaymentsForLedgerCommand),
    ForTransaction(PaymentsForTransactionCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of payments")]
pub struct AllPaymentsCommand {
    #[structopt(long, help = "Include failed payments")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of payments filtered by account")]
pub struct PaymentsForAccountCommand {
    #[structopt(name = "ACCOUNT_ID", help = "The account id")]
    pub account_id: String,
    #[structopt(long, help = "Include failed payments")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of payments filtered by ledger")]
pub struct PaymentsForLedgerCommand {
    #[structopt(name = "LEDGER_ID", help = "The ledger id")]
    pub ledger_id: LedgerId,
    #[structopt(long, help = "Include failed payments")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of payments filtered by transaction")]
pub struct PaymentsForTransactionCommand {
    #[structopt(name = "TRANSACTION_ID", help = "The transaction id")]
    pub transaction_id: String,
    #[structopt(flatten)]
    pub paging: Paging,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: PaymentCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        PaymentCommand::All(cmd) => run_all(&mut out, &config, client, cmd).await,
        PaymentCommand::ForAccount(cmd) => run_for_account(&mut out, &config, client, cmd).await,
        PaymentCommand::ForLedger(cmd) => run_for_ledger(&mut out, &config, client, cmd).await,
        PaymentCommand::ForTransaction(cmd) => {
            run_for_transaction(&mut out, &config, client, cmd).await
        }
    }
}

pub async fn run_all<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllPaymentsCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::payments::all().with_include_failed(command.include_failed);
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
    command: PaymentsForAccountCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let account = PublicKey::from_account_id(&command.account_id)?;
    let request = api::payments::for_account(&account).with_include_failed(command.include_failed);
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
    command: PaymentsForLedgerCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request =
        api::payments::for_ledger(command.ledger_id).with_include_failed(command.include_failed);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}

pub async fn run_for_transaction<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: PaymentsForTransactionCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::payments::for_transaction(command.transaction_id);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}
