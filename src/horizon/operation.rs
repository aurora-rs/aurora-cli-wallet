use crate::config::AppConfig;
use crate::horizon::{execute_and_print_page_request, execute_and_print_request, Paging};
use anyhow::Result;
use convey::Output;
use stellar_base::PublicKey;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Horizon operation endpoints")]
pub enum OperationCommand {
    All(AllOperationsCommand),
    Single(SingleOperationCommand),
    ForAccount(OperationsForAccountCommand),
    ForLedger(OperationsForLedgerCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a single operation")]
pub struct SingleOperationCommand {
    #[structopt(name = "OPERATION_ID", help = "The operation id")]
    pub operation_id: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of operations")]
pub struct AllOperationsCommand {
    #[structopt(long, help = "Include failed operations")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of operations filtered by account")]
pub struct OperationsForAccountCommand {
    #[structopt(name = "ACCOUNT_ID", help = "The account id")]
    pub account_id: String,
    #[structopt(long, help = "Include failed operations")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of operations filtered by ledger")]
pub struct OperationsForLedgerCommand {
    #[structopt(name = "LEDGER_ID", help = "The ledger id")]
    pub ledger_id: i32,
    #[structopt(long, help = "Include failed operations")]
    pub include_failed: bool,
    #[structopt(flatten)]
    pub paging: Paging,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: OperationCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        OperationCommand::All(cmd) => run_all(&mut out, &config, client, cmd).await,
        OperationCommand::Single(cmd) => run_single(&mut out, &config, client, cmd).await,
        OperationCommand::ForAccount(cmd) => run_for_account(&mut out, &config, client, cmd).await,
        OperationCommand::ForLedger(cmd) => run_for_ledger(&mut out, &config, client, cmd).await,
    }
}

pub async fn run_all<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllOperationsCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::operations::all().with_include_failed(command.include_failed);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}

pub async fn run_single<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: SingleOperationCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::operations::single(command.operation_id);
    execute_and_print_request(&mut out, client, request).await
}

pub async fn run_for_account<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: OperationsForAccountCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let account = PublicKey::from_account_id(&command.account_id)?;
    let request =
        api::operations::for_account(&account).with_include_failed(command.include_failed);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}

pub async fn run_for_ledger<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: OperationsForLedgerCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request =
        api::operations::for_ledger(command.ledger_id).with_include_failed(command.include_failed);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}
