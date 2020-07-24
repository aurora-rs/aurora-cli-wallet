use crate::config::AppConfig;
use crate::horizon::{execute_and_print_page_request, execute_and_print_request, Paging};
use anyhow::Result;
use convey::Output;
use stellar_base::PublicKey;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Horizon transaction endpoints")]
pub enum TransactionCommand {
    All(AllTransactionsCommand),
    Single(SingleTransactionCommand),
    ForAccount(TransactionsForAccountCommand),
    ForLedger(TransactionsForLedgerCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a single transaction")]
pub struct SingleTransactionCommand {
    #[structopt(name = "TRANSACTION_ID", help = "The transaction id")]
    pub transaction_id: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions")]
pub struct AllTransactionsCommand {
    #[structopt(long, help = "Include failed transactions")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions filtered by account")]
pub struct TransactionsForAccountCommand {
    #[structopt(name = "ACCOUNT_ID", help = "The account id")]
    pub account_id: String,
    #[structopt(long, help = "Include failed transactions")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of transactions filtered by ledger")]
pub struct TransactionsForLedgerCommand {
    #[structopt(name = "LEDGER_ID", help = "The ledger id")]
    pub ledger_id: u32,
    #[structopt(long, help = "Include failed transactions")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: TransactionCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        TransactionCommand::All(cmd) => run_all(&mut out, &config, client, cmd).await,
        TransactionCommand::Single(cmd) => run_single(&mut out, &config, client, cmd).await,
        TransactionCommand::ForAccount(cmd) => {
            run_for_account(&mut out, &config, client, cmd).await
        }
        TransactionCommand::ForLedger(cmd) => run_for_ledger(&mut out, &config, client, cmd).await,
    }
}

pub async fn run_all<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllTransactionsCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::transactions::all().with_include_failed(command.include_failed);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}

pub async fn run_single<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: SingleTransactionCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::transactions::single(command.transaction_id);
    execute_and_print_request(&mut out, client, request).await
}

pub async fn run_for_account<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: TransactionsForAccountCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let account = PublicKey::from_account_id(&command.account_id)?;
    let request =
        api::transactions::for_account(&account).with_include_failed(command.include_failed);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}

pub async fn run_for_ledger<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: TransactionsForLedgerCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::transactions::for_ledger(command.ledger_id)
        .with_include_failed(command.include_failed);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}
