use crate::config::AppConfig;
use crate::error::Error;
use crate::render::ResponseRender;
use anyhow::Result;
use convey::Output;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use stellar_horizon::request::{Order, PageRequest};
use structopt::StructOpt;
use tokio::stream::StreamExt;

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
    pub ledger_id: i32,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of ledgers")]
pub struct AllLedgersCommand {
    #[structopt(long, help = "Start from this ledger")]
    pub cursor: Option<String>,
    #[structopt(long, help = "Limit the number of ledgers returned")]
    pub limit: Option<u64>,
    #[structopt(long, help = "Return ledgers in ascending order", group = "order")]
    pub ascending: bool,
    #[structopt(long, help = "Return ledgers in descending order", group = "order")]
    pub descending: bool,
    #[structopt(long, help = "Stream response")]
    pub stream: bool,
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
    out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllLedgersCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let mut request = api::ledgers::all();

    if let Some(cursor) = command.cursor {
        request = request.with_cursor(&cursor);
    }

    if let Some(limit) = command.limit {
        request = request.with_limit(limit);
    }

    if command.ascending {
        request = request.with_order(&Order::Ascending);
    }

    if command.descending {
        request = request.with_order(&Order::Descending);
    }

    if command.stream {
        let mut stream = client.stream(request)?;
        while let Some(event) = stream.try_next().await? {
            out.print(ResponseRender(event)).map_err(Error::Convey)?;
        }
        Ok(())
    } else {
        let (_, response) = client.request(request).await?;
        // out.print(ResponseRender(response)).map_err(Error::Convey)?;
        for record in response.records {
            out.print(ResponseRender(record)).map_err(Error::Convey)?;
        }
        Ok(())
    }
}

pub async fn run_single<H>(
    out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: SingleLedgerCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::ledgers::single(command.ledger_id);
    let (_, response) = client.request(request).await?;
    out.print(ResponseRender(response)).map_err(Error::Convey)?;
    Ok(())
}
