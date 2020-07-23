use crate::config::{self, AppConfig, Server};
use crate::error::Error;
use anyhow::Result;
use convey::Output;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage Horizon servers")]
pub enum ServerCommand {
    #[structopt(about = "Add a new Horizon server")]
    Add(AddCommand),
    #[structopt(about = "Removes an Horizon server")]
    Remove(RemoveCommand),
    #[structopt(about = "Lists all Horizon servers")]
    List,
}

#[derive(Debug, StructOpt)]
pub struct AddCommand {
    #[structopt(name = "NAME", help = "The server name")]
    pub name: String,
    #[structopt(name = "URI", help = "The server URI")]
    pub uri: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveCommand {
    #[structopt(name = "NAME", help = "The server name")]
    pub name: String,
}

pub fn run_command(
    mut out: &mut Output,
    mut config: &mut AppConfig,
    command: ServerCommand,
) -> Result<()> {
    match command {
        ServerCommand::Add(cmd) => run_add(&mut out, &mut config, cmd),
        ServerCommand::Remove(cmd) => run_remove(&mut out, &mut config, cmd),
        ServerCommand::List => run_list(&mut out, &config),
    }
}

pub fn run_add(out: &mut Output, config: &mut AppConfig, command: AddCommand) -> Result<()> {
    url::Url::parse(&command.uri)?;
    let server = Server {
        name: command.name,
        uri: command.uri,
    };
    let existing = config.servers.iter().find(|s| s.name == server.name);
    match existing {
        None => config.servers.push(server.clone()),
        Some(_) => {}
    };
    config::store(&config)?;
    out.print(server).map_err(Error::Convey)?;
    Ok(())
}

pub fn run_remove(_out: &mut Output, config: &mut AppConfig, command: RemoveCommand) -> Result<()> {
    config.servers.retain(|s| s.name != command.name);
    config::store(&config)?;
    Ok(())
}

pub fn run_list(out: &mut Output, config: &AppConfig) -> Result<()> {
    let server_list = config::ServerList {
        servers: config.servers.clone(),
    };
    out.print(server_list).map_err(Error::Convey)?;
    Ok(())
}
