use crate::config::AppConfig;
use crate::error::Error;
use crate::render::ResponseRender;
use anyhow::Result;
use convey::Output;
use serde::ser::Serialize;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use stellar_horizon::error::Error as HorizonError;
use stellar_horizon::request::{Order, PageRequest, Request, StreamRequest};
use structopt::StructOpt;
use tokio_stream::StreamExt;

mod account;
mod claimable_balance;
mod effect;
mod ledger;
mod operation;
mod payment;
mod server;
mod trade;
mod transaction;

#[derive(Debug, StructOpt)]
pub struct Paging {
    #[structopt(long, help = "Start from this ledger")]
    pub cursor: Option<String>,
    #[structopt(long, help = "Limit the number of ledgers returned")]
    pub limit: Option<u64>,
    #[structopt(long, help = "Return ledgers in ascending order", group = "order")]
    pub ascending: bool,
    #[structopt(long, help = "Return ledgers in descending order", group = "order")]
    pub descending: bool,
}

#[derive(Debug, StructOpt)]
pub struct Streaming {
    #[structopt(long, help = "Stream response")]
    pub stream: bool,
}

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
    ClaimableBalance(claimable_balance::ClaimableBalanceCommand),
    Ledger(ledger::LedgerCommand),
    Operation(operation::OperationCommand),
    Payment(payment::PaymentCommand),
    Transaction(transaction::TransactionCommand),
    Effect(effect::EffectCommand),
    Trade(trade::TradeCommand),
    Info,
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
                HorizonNonServerCommand::ClaimableBalance(cmd) => {
                    claimable_balance::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Ledger(cmd) => {
                    ledger::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Operation(cmd) => {
                    operation::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Payment(cmd) => {
                    payment::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Transaction(cmd) => {
                    transaction::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Effect(cmd) => {
                    effect::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Trade(cmd) => {
                    trade::run_command(&mut out, &config, &client, cmd).await
                }
                HorizonNonServerCommand::Info => {
                    execute_and_print_request(&mut out, &client, api::root::root()).await
                }
            }
        }
    }
}

pub fn add_paging_options<R: PageRequest>(mut request: R, options: &Paging) -> R {
    if let Some(cursor) = options.cursor.as_ref() {
        request = request.with_cursor(cursor);
    }

    if let Some(limit) = options.limit {
        request = request.with_limit(limit);
    }

    if options.ascending {
        request = request.with_order(&Order::Ascending);
    }

    if options.descending {
        request = request.with_order(&Order::Descending);
    }

    request
}

pub async fn execute_and_print_request<H, R>(out: &mut Output, client: &H, request: R) -> Result<()>
where
    H: HorizonClient,
    R: Request,
    R::Response: Serialize,
{
    match client.request(request).await {
        Ok((_, response)) => {
            out.print(ResponseRender(response)).map_err(Error::Convey)?;
            Ok(())
        }
        Err(HorizonError::HorizonRequestError(err)) => {
            out.print(ResponseRender(err.clone()))
                .map_err(Error::Convey)?;
            Err(HorizonError::HorizonRequestError(err))?
        }
        Err(err) => Err(err)?,
    }
}

pub async fn execute_and_print_page_request<H, R>(
    mut out: &mut Output,
    client: &H,
    mut request: R,
    paging: &Paging,
) -> Result<()>
where
    H: HorizonClient,
    R: PageRequest + 'static,
    R::Response: Serialize,
{
    request = add_paging_options(request, &paging);
    execute_and_print_request(&mut out, client, request).await
}

pub async fn execute_and_print_stream_request<H, R>(
    mut out: &mut Output,
    client: &H,
    mut request: R,
    paging: &Paging,
    streaming: &Streaming,
) -> Result<()>
where
    H: HorizonClient,
    R: StreamRequest + PageRequest + 'static,
    R::Response: Serialize,
    R::Resource: Serialize,
{
    request = add_paging_options(request, &paging);

    if streaming.stream {
        let mut stream = client.stream(request)?;
        while let Some(event) = stream.try_next().await? {
            out.print(ResponseRender(event)).map_err(Error::Convey)?;
        }
        Ok(())
    } else {
        execute_and_print_request(&mut out, client, request).await
    }
}
