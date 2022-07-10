mod accounts_db_impl;
pub mod clients_db_impl;
pub mod project_db_impl;
pub mod tasks_db_impl;
use mongodb::bson::doc;

use crate::Result;
use mongodb::{options::ClientOptions, Client};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub db_name: String,
}

impl DB {
    pub async fn init(db_url: &str, db_name: String) -> Result<Self> {
        let mut client_options = ClientOptions::parse(db_url).await?;
        client_options.app_name = Some(db_name.clone());

        Ok(Self {
            client: Client::with_options(client_options)?,
            db_name: db_name.clone(),
        })
    }

    // pub async fn drop_all_collections(&self) {
    //     self.client.database(&self.db_name).drop(None).await?
    // }

    /// Example on how to get all collection names (for droping them for example)
    pub async fn get_all_collection_names(&self) {
        for collection_name in self
            .client
            .database(&self.db_name)
            .list_collection_names(None)
            .await
        {
            println!("{:?}", collection_name);
        }
    }
}
