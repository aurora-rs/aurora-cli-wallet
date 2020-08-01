use crate::config::AppConfig;
use crate::horizon::{
    execute_and_print_page_request, execute_and_print_stream_request, Paging, Streaming,
};
use anyhow::Result;
use convey::Output;
use stellar_base::PublicKey;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use stellar_horizon::resources::OfferId;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Horizon trade endpoints")]
pub enum TradeCommand {
    All(AllTradesCommand),
    ForAccount(TradesForAccountCommand),
    ForOffer(TradesForOfferCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of trades")]
pub struct AllTradesCommand {
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of trades filtered by account")]
pub struct TradesForAccountCommand {
    #[structopt(name = "ACCOUNT_ID", help = "The account id")]
    pub account_id: String,
    #[structopt(flatten)]
    pub paging: Paging,
    #[structopt(flatten)]
    pub streaming: Streaming,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a list of trades filtered by offer")]
pub struct TradesForOfferCommand {
    #[structopt(name = "OFFER_ID", help = "The offer id")]
    pub offer_id: OfferId,
    #[structopt(flatten)]
    pub paging: Paging,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: TradeCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        TradeCommand::All(cmd) => run_all(&mut out, &config, client, cmd).await,
        TradeCommand::ForAccount(cmd) => run_for_account(&mut out, &config, client, cmd).await,
        TradeCommand::ForOffer(cmd) => run_for_offer(&mut out, &config, client, cmd).await,
    }
}

pub async fn run_all<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllTradesCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::trades::all();
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
    command: TradesForAccountCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let account = PublicKey::from_account_id(&command.account_id)?;
    let request = api::trades::for_account(&account);
    execute_and_print_stream_request(
        &mut out,
        client,
        request,
        &command.paging,
        &command.streaming,
    )
    .await
}

pub async fn run_for_offer<H>(
    mut out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: TradesForOfferCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::trades::for_offer(command.offer_id);
    execute_and_print_page_request(&mut out, client, request, &command.paging).await
}
