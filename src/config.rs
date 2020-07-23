use anyhow::Result;
use convey::components::{newline, text};
use convey::Render;
use stellar_base::KeyPair;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub account_id: String,
    pub secret_seed: String,
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        AppConfig { accounts: vec![] }
    }
}

pub fn load() -> Result<AppConfig> {
    Ok(confy::load("aurora")?)
}

pub fn store(config: &AppConfig) -> Result<()> {
    Ok(confy::store("aurora", &config)?)
}

impl Account {
    pub fn new(keypair: &KeyPair) -> Account {
        Account {
            account_id: keypair.public_key().account_id(),
            secret_seed: keypair.secret_key().secret_seed(),
        }
    }
}

impl Render for Account {
    render_for_humans!(self -> [
        text("Account ID: "), text(&self.account_id), newline(),
        text("Secret Seed: "), text(&self.secret_seed), newline(),
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
            fmt.write("\n".as_bytes())?;
        }
        Ok(())
    }

    render_json!();
}
