use clap::Parser;
use dotenv;
use std::env;

use crate::error::Error;

/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "5000")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "mongoadmin")]
    pub db_user: String,
    /// Database user
    #[clap(long)]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "127.0.0.1")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "27017")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "rust-time-tracker-base-v2")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        dotenv::dotenv().ok();
        let config = Config::parse();

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(|e| Error::ParseError(e))?;

        let db_user = env::var("MONGODB_USER").unwrap_or(config.db_user.to_owned());
        let db_password = env::var("MONGODB_PASSWORD").unwrap();
        let db_host = env::var("MONGODB_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = env::var("MONGODB_PORT").unwrap_or(config.db_port.to_string());
        let db_name = env::var("MONGODB_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port.parse::<u16>().map_err(|e| Error::ParseError(e))?,
            db_name,
        })
    }
}
