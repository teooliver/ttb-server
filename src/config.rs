use clap::Parser;
use std::env;

use crate::handle_errors::Error;

/// Q&A web service API
#[derive(Parser, Debug, PartialEq)]
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
    #[clap(long, default_value = "password")]
    pub db_password: String,
    /// URL for the mongodb database
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
        let config = Config::parse();

        if env::var("PASETO_KEY").is_err() {
            panic!("PASETO_KEY not set");
        }

        let port = env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(|e| Error::ParseError(e))?;

        let db_user = env::var("MONGODB_USER").unwrap_or_else(|_| config.db_user.to_owned());
        let db_password = env::var("MONGODB_PASSWORD").unwrap();
        let db_host = env::var("MONGODB_HOST").unwrap_or_else(|_| config.db_host.to_owned());
        let db_port = env::var("MONGODB_PORT").unwrap_or_else(|_| config.db_port.to_string());
        let db_name = env::var("MONGODB_DB").unwrap_or_else(|_| config.db_name.to_owned());

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

#[cfg(test)]
mod config_tests {
    use super::*;

    fn set_env() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        env::set_var("PORT", "5000");
        env::set_var("MONGODB_USER", "mongoadmin");
        env::set_var("MONGODB_PASSWORD", "secret");
        env::set_var("MONGODB_HOST", "127.0.0.1");
        env::set_var("MONGODB_PORT", "27017");
        env::set_var("MONGODB_DB", "test_db");
    }

    #[test]
    fn unset_and_set_api_key() {
        // ENV VARIABLES ARE NOT SET
        let result = std::panic::catch_unwind(|| Config::new());
        assert!(result.is_err());

        // NOW WE SET THEM
        set_env();

        let expected = Config {
            log_level: "warn".to_string(),
            port: 5000,
            db_user: "mongoadmin".to_string(),
            db_password: "secret".to_string(),
            db_host: "127.0.0.1".to_string(),
            db_port: 27017,
            db_name: "test_db".to_string(),
        };
        let config = Config::new().unwrap();

        assert_eq!(config, expected);
    }
}
