use crate::config::{self, Account, AppConfig};
use crate::error::Error;
use anyhow::Result;
use convey::Output;
use stellar_base::KeyPair;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage Stellar accounts")]
pub enum AccountCommand {
    #[structopt(about = "Creates a new random account")]
    New,
    #[structopt(about = "Creates a new account")]
    Add(AddCommand),
    #[structopt(about = "Removes an account")]
    Remove(RemoveCommand),
    #[structopt(about = "Lists all accounts")]
    List,
}

#[derive(Debug, StructOpt)]
pub struct AddCommand {
    #[structopt(name = "SEED", help = "The Stellar account secret seed, starts with S")]
    secret_seed: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveCommand {
    #[structopt(name = "ACCOUNT_ID", help = "The Stellar account id, starts with G")]
    account_id: String,
}

pub fn run_command(
    mut out: &mut Output,
    mut config: &mut AppConfig,
    command: AccountCommand,
) -> Result<()> {
    match command {
        AccountCommand::New => run_new(&mut out, &mut config),
        AccountCommand::Add(cmd) => run_add(&mut out, &mut config, cmd),
        AccountCommand::Remove(cmd) => run_remove(&mut out, &mut config, cmd),
        AccountCommand::List => run_list(&mut out, &config),
    }
}

pub fn run_new(mut out: &mut Output, mut config: &mut AppConfig) -> Result<()> {
    let keypair = KeyPair::random()?;
    add_keypair_to_config(&mut out, &mut config, &keypair)
}

pub fn run_add(
    mut out: &mut Output,
    mut config: &mut AppConfig,
    command: AddCommand,
) -> Result<()> {
    let keypair = KeyPair::from_secret_seed(&command.secret_seed)?;
    add_keypair_to_config(&mut out, &mut config, &keypair)
}

pub fn run_remove(_out: &mut Output, config: &mut AppConfig, command: RemoveCommand) -> Result<()> {
    config
        .accounts
        .retain(|a| a.account_id != command.account_id);
    config::store(&config)?;
    Ok(())
}

pub fn run_list(out: &mut Output, config: &AppConfig) -> Result<()> {
    let account_list = config::AccountList {
        accounts: config.accounts.clone(),
    };
    out.print(account_list).map_err(Error::Convey)?;
    Ok(())
}

fn add_keypair_to_config(
    out: &mut Output,
    config: &mut AppConfig,
    keypair: &KeyPair,
) -> Result<()> {
    let account = Account::new(&keypair);
    match config
        .accounts
        .iter()
        .find(|a| a.account_id == account.account_id)
    {
        None => config.accounts.push(account.clone()),
        Some(_) => {}
    };
    config::store(&config)?;
    out.print(account).map_err(Error::Convey)?;
    Ok(())
}
