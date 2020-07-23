use anyhow::Result;
use convey::components::{newline, text};
use convey::Render;
use stellar_base::KeyPair;
use stellar_horizon::client::HorizonHttpClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub default_server: String,
    pub accounts: Vec<Account>,
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub account_id: String,
    pub secret_seed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub name: String,
    pub uri: String,
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        let servers = vec![
            Server::new("public", "https://horizon.stellar.org"),
            Server::new("test", "https://horizon-testnet.stellar.org"),
        ];
        AppConfig {
            accounts: vec![],
            default_server: "test".to_string(),
            servers,
        }
    }
}

pub fn load() -> Result<AppConfig> {
    Ok(confy::load("aurora")?)
}

pub fn store(config: &AppConfig) -> Result<()> {
    Ok(confy::store("aurora", &config)?)
}

impl AppConfig {
    pub fn horizon_client_for_server(&self, server_name: &str) -> Result<HorizonHttpClient> {
        let server = self
            .servers
            .iter()
            .find(|s| s.name == server_name)
            .ok_or_else(|| anyhow!("Invalid server name"))?;
        Ok(HorizonHttpClient::new_from_str(&server.uri)?)
    }
}

impl Account {
    pub fn new(keypair: &KeyPair) -> Account {
        Account {
            account_id: keypair.public_key().account_id(),
            secret_seed: keypair.secret_key().secret_seed(),
        }
    }
}

impl Server {
    pub fn new<S: Into<String>>(name: S, uri: S) -> Server {
        Server {
            name: name.into(),
            uri: uri.into(),
        }
    }
}

impl Render for Account {
    render_for_humans!(self -> [
        text(&self.account_id), text("\t"), text(&self.secret_seed), newline(),
    ]);

    render_json!();
}

impl Render for Server {
    render_for_humans!(self -> [
        text(&self.name), text("\t"), text(&self.uri), newline(),
    ]);

    render_json!();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountList {
    pub accounts: Vec<Account>,
}

impl Render for AccountList {
    fn render_for_humans(
        &self,
        mut fmt: &mut convey::human::Formatter,
    ) -> std::result::Result<(), convey::Error> {
        for account in &self.accounts {
            account.render_for_humans(&mut fmt)?;
        }
        Ok(())
    }

    render_json!();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerList {
    pub servers: Vec<Server>,
}

impl Render for ServerList {
    fn render_for_humans(
        &self,
        mut fmt: &mut convey::human::Formatter,
    ) -> std::result::Result<(), convey::Error> {
        for server in &self.servers {
            server.render_for_humans(&mut fmt)?;
        }
        Ok(())
    }

    render_json!();
}
