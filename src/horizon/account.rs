use crate::config::AppConfig;
use crate::error::Error;
use crate::render::ResponseRender;
use anyhow::Result;
use convey::Output;
use stellar_base::PublicKey;
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Horizon account endpoints")]
pub enum AccountCommand {
    Single(SingleAccountCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a single Stellar account")]
pub struct SingleAccountCommand {
    #[structopt(name = "ACCOUNT_ID", help = "The Stellar account id, starts with G")]
    pub account_id: String,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: AccountCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        AccountCommand::Single(cmd) => run_single(&mut out, &config, client, cmd).await,
    }
}

pub async fn run_single<H>(
    out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: SingleAccountCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let account = PublicKey::from_account_id(&command.account_id)?;
    let request = api::accounts::single(&account);
    let (_, response) = client.request(request).await?;
    out.print(ResponseRender(response)).map_err(Error::Convey)?;
    Ok(())
}
