//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use dotenv::dotenv;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum Database {
    Mysql,
    MongoDB
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub server: String,
    pub database: Database,
    pub database_host: String,
    pub database_port: u16,
    pub database_account: String,
    pub database_password: String,
    pub database_db: String,
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
