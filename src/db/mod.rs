pub mod clients_db_impl;
pub mod project_db_impl;
pub mod tasks_db_impl;

use crate::Result;
use mongodb::{options::ClientOptions, Client};

pub const DB_NAME: &str = "rust-time-tracker-base-v2";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some(DB_NAME.to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }
}
