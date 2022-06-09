pub mod clients_db_impl;
pub mod project_db_impl;
pub mod tasks_db_impl;

use crate::Result;
use mongodb::{options::ClientOptions, Client};

// pub const DB_NAME: &str = "rust-time-tracker-base-v2";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub db_name: String,
}

impl DB {
    pub async fn init(db_url: &str, db_name: String) -> Result<Self> {
        // let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        // client_options.app_name = Some(DB_NAME.to_string());
        let mut client_options = ClientOptions::parse(db_url).await?;
        client_options.app_name = Some(db_name.clone());

        Ok(Self {
            client: Client::with_options(client_options)?,
            db_name: db_name.clone(),
        })
    }
}
