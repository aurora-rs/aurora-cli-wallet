use crate::config::AppConfig;
use crate::error::Error;
use crate::render::ResponseRender;
use anyhow::Result;
use convey::Output;
use stellar_base::{Asset, PublicKey};
use stellar_horizon::api;
use stellar_horizon::client::HorizonClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Horizon claimable balance endpoints")]
pub enum ClaimableBalanceCommand {
    AllByClaimant(AllByClaimantClaimableBalanceCommand),
    AllBySponsor(AllBySponsorClaimableBalanceCommand),
    AllByAsset(AllByAssetClaimableBalanceCommand),
    Single(SingleClaimableBalanceCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about all claimable balances filtered by asset")]
pub struct AllByAssetClaimableBalanceCommand {
    #[structopt(
        name = "ASSET",
        help = "The claimable balance claimant asset. Use XLM for the native asset, and CODE:ISSUER for all other assets."
    )]
    pub asset: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about all claimable balances filtered by claimant")]
pub struct AllByClaimantClaimableBalanceCommand {
    #[structopt(
        name = "CLAIMANT_ID",
        help = "The claimable balance claimant account id"
    )]
    pub claimant_id: String,
}

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Retrieves information about all claimable balances filtered by sponsoring account id"
)]
pub struct AllBySponsorClaimableBalanceCommand {
    #[structopt(name = "SPONSOR_ID", help = "The claimable balance sponsor account id")]
    pub sponsor_id: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieves information about a single claimable balance")]
pub struct SingleClaimableBalanceCommand {
    #[structopt(name = "CLAIMABLE_BALANCE_ID", help = "The claimable balance id")]
    pub claimable_balance_id: String,
}

pub async fn run_command<H>(
    mut out: &mut Output,
    config: &AppConfig,
    client: &H,
    command: ClaimableBalanceCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    match command {
        ClaimableBalanceCommand::AllByAsset(cmd) => {
            run_all_by_asset(&mut out, &config, client, cmd).await
        }
        ClaimableBalanceCommand::AllByClaimant(cmd) => {
            run_all_by_claimant(&mut out, &config, client, cmd).await
        }
        ClaimableBalanceCommand::AllBySponsor(cmd) => {
            run_all_by_sponsor(&mut out, &config, client, cmd).await
        }
        ClaimableBalanceCommand::Single(cmd) => run_single(&mut out, &config, client, cmd).await,
    }
}

pub async fn run_all_by_asset<H>(
    out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllByAssetClaimableBalanceCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let asset = parse_asset(command.asset)?;
    let request = api::claimable_balances::all_by_asset(asset);
    let (_, response) = client.request(request).await?;
    out.print(ResponseRender(response)).map_err(Error::Convey)?;
    Ok(())
}

pub async fn run_all_by_claimant<H>(
    out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllByClaimantClaimableBalanceCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let claimant = PublicKey::from_account_id(&command.claimant_id)?;
    let request = api::claimable_balances::all_by_claimant(&claimant);
    let (_, response) = client.request(request).await?;
    out.print(ResponseRender(response)).map_err(Error::Convey)?;
    Ok(())
}

pub async fn run_all_by_sponsor<H>(
    out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: AllBySponsorClaimableBalanceCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let sponsor = PublicKey::from_account_id(&command.sponsor_id)?;
    let request = api::claimable_balances::all_by_sponsor(&sponsor);
    let (_, response) = client.request(request).await?;
    out.print(ResponseRender(response)).map_err(Error::Convey)?;
    Ok(())
}

pub async fn run_single<H>(
    out: &mut Output,
    _config: &AppConfig,
    client: &H,
    command: SingleClaimableBalanceCommand,
) -> Result<()>
where
    H: HorizonClient,
{
    let request = api::claimable_balances::single(command.claimable_balance_id);
    let (_, response) = client.request(request).await?;
    out.print(ResponseRender(response)).map_err(Error::Convey)?;
    Ok(())
}

fn parse_asset(asset: String) -> Result<Asset> {
    let parts: Vec<&str> = asset.split(":").collect();
    if parts.len() == 1 {
        if parts[0].to_ascii_uppercase() == "XLM" {
            return Ok(Asset::new_native());
        }
    }
    if parts.len() == 2 {
        let issuer = PublicKey::from_account_id(parts[1])?;
        let result = Asset::new_credit(parts[0], issuer)?;
        return Ok(result);
    }
    return Err(anyhow!("Invalid asset format"));
}
