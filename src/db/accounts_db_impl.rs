use crate::handle_errors;
use crate::handle_errors::Error::*;
use crate::models::account::Account;
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{self, Bson};
use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::options::IndexOptions;
use mongodb::{Collection, IndexModel};
use std::error::Error;

use super::DB;

impl DB {
    pub fn get_accounts_collection(&self) -> Collection<Document> {
        self.client.database(&self.db_name).collection("accounts")
    }

    // pub async fn create_accounts_indexes(&self) {
    //     let options = IndexOptions::builder().unique(true).build();
    //     let model = IndexModel::builder()
    //         .keys(doc! {"email": 1})
    //         .options(options)
    //         .build();

    //     self.get_accounts_collection()
    //         .create_index(model, None)
    //         .await
    //         .expect("error creating index!");
    // }

    fn doc_to_account(&self, doc: &Document) -> Result<Account, handle_errors::Error> {
        //      pub _id: Option<ObjectId>,
        // pub first_name: String,
        // pub last_name: String,
        // pub email: String,
        // pub password: String,
        // pub created_at: Option<DateTime>,
        // pub updated_at: Option<DateTime>,

        let id = doc.get_object_id("_id")?;
        let first_name = doc.get_str("first_name").unwrap_or_else(|_| "");
        let last_name = doc.get_str("last_name").unwrap_or_else(|_| "");
        let email = doc.get_str("email")?;
        let password = doc.get_str("password")?;
        let created_at = doc.get_datetime("created_at")?;
        let updated_at = doc.get_datetime("updated_at")?;

        let account = Account {
            _id: Some(id),
            first_name: Some(first_name.to_owned()),
            last_name: Some(last_name.to_owned()),
            email: email.to_owned(),
            password: password.to_owned(),
            created_at: Some(*created_at),
            updated_at: Some(*updated_at),
        };

        Ok(account)
    }

    pub async fn create_account(&self, _entry: &Account) -> Result<Bson, handle_errors::Error> {
        let new_account = self
            .get_accounts_collection()
            .insert_one(
                doc! {
                "email": _entry.email.clone(),
                "password": _entry.password.clone(),
                "created_at": chrono::Utc::now().clone(),
                "updated_at": chrono::Utc::now().clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(new_account.inserted_id)
    }

    pub async fn get_account(&self, email: &str) -> Result<Account, handle_errors::Error> {
        let query = doc! {
            "email": email,
        };

        let document = self
            .get_accounts_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        if document.is_none() {
            return Err(ObjNotFound);
        }

        let result = self.doc_to_account(&document.unwrap())?;

        Ok(result)
    }
}
